//! Bounded ZIP bundle operations for ReproPack 0.1.
//!
//! This module treats archives and their metadata as untrusted input. It never
//! executes bundle contents.

use crate::{Manifest, ModelError, ValidationError};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use zip::read::ZipArchive;
use zip::result::ZipError;
use zip::write::{SimpleFileOptions, ZipWriter};
use zip::CompressionMethod;

pub const MANIFEST_PATH: &str = "manifest.json";
pub const EVIDENCE_PREFIX: &str = "evidence/";

#[derive(Debug, Clone, Copy)]
pub struct ReadLimits {
    pub max_manifest_bytes: u64,
    pub max_entries: usize,
    pub max_entry_bytes: u64,
    pub max_total_bytes: u64,
    pub max_path_bytes: usize,
}

impl Default for ReadLimits {
    fn default() -> Self {
        Self {
            max_manifest_bytes: 1024 * 1024,
            max_entries: 10_000,
            max_entry_bytes: 256 * 1024 * 1024,
            max_total_bytes: 1024 * 1024 * 1024,
            max_path_bytes: 4096,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bundle {
    pub manifest: Manifest,
    pub evidence: BTreeMap<String, Vec<u8>>,
}

#[derive(Debug, Error)]
pub enum BundleError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] ZipError),
    #[error(transparent)]
    Model(#[from] ModelError),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("archive is malformed: {0}")]
    Malformed(String),
    #[error("unsafe archive path: {0}")]
    UnsafePath(String),
    #[error("duplicate archive entry: {0}")]
    DuplicateEntry(String),
    #[error("archive entry is encrypted: {0}")]
    EncryptedEntry(String),
    #[error("archive entry is a directory or special file: {0}")]
    UnsupportedEntryType(String),
    #[error("archive limit exceeded for {0}")]
    LimitExceeded(String),
    #[error("manifest entry is missing from archive: {0}")]
    MissingEntry(String),
    #[error("archive contains unindexed evidence: {0}")]
    UnexpectedEntry(String),
    #[error("hash mismatch for {0}")]
    HashMismatch(String),
    #[error("size mismatch for {0}")]
    SizeMismatch(String),
    #[error("extraction refused for unsafe destination path: {0}")]
    UnsafeDestination(String),
    #[error("capture mapping must use archive_path=source_path: {0}")]
    InvalidCaptureMapping(String),
}

pub fn create_bundle<W: Write + Seek>(
    writer: W,
    manifest: &Manifest,
    evidence: &BTreeMap<String, Vec<u8>>,
) -> Result<W, BundleError> {
    manifest.validate()?;
    ensure_correspondence(manifest, evidence.keys())?;
    let mut archive = ZipWriter::new(writer);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    archive.start_file(MANIFEST_PATH, options)?;
    archive.write_all(manifest.to_canonical_json()?.as_bytes())?;
    for (path, bytes) in evidence {
        archive.start_file(path, options)?;
        archive.write_all(bytes)?;
    }
    Ok(archive.finish()?)
}

pub fn read_bundle<R: Read + Seek>(reader: R, limits: ReadLimits) -> Result<Bundle, BundleError> {
    let mut archive = ZipArchive::new(reader)?;
    if archive.len() > limits.max_entries {
        return Err(BundleError::LimitExceeded("entry count".into()));
    }
    let mut names = HashSet::new();
    let mut manifest_bytes = None;
    let mut evidence = BTreeMap::new();
    let mut total_bytes = 0_u64;

    for index in 0..archive.len() {
        let entry = archive.by_index(index)?;
        let name = entry.name().to_string();
        if name.len() > limits.max_path_bytes || !is_safe_archive_path(&name) {
            return Err(BundleError::UnsafePath(name));
        }
        if !names.insert(name.clone()) {
            return Err(BundleError::DuplicateEntry(name));
        }
        if entry.encrypted() {
            return Err(BundleError::EncryptedEntry(name));
        }
        if entry.is_dir() || is_special_file(&entry) {
            return Err(BundleError::UnsupportedEntryType(name));
        }
        let size = entry.size();
        if size > limits.max_entry_bytes {
            return Err(BundleError::LimitExceeded(name));
        }
        total_bytes = total_bytes
            .checked_add(size)
            .ok_or_else(|| BundleError::LimitExceeded("total bytes".into()))?;
        if total_bytes > limits.max_total_bytes {
            return Err(BundleError::LimitExceeded("total bytes".into()));
        }
        let mut bytes = Vec::with_capacity(size.min(1024 * 1024) as usize);
        let read_limit = size
            .checked_add(1)
            .ok_or_else(|| BundleError::LimitExceeded("entry size".into()))?;
        entry.take(read_limit).read_to_end(&mut bytes)?;
        if bytes.len() as u64 != size {
            return Err(BundleError::Malformed(format!("truncated entry: {name}")));
        }
        if name == MANIFEST_PATH {
            if bytes.len() as u64 > limits.max_manifest_bytes {
                return Err(BundleError::LimitExceeded("manifest bytes".into()));
            }
            manifest_bytes = Some(bytes);
        } else if name.starts_with(EVIDENCE_PREFIX) {
            evidence.insert(name, bytes);
        } else {
            return Err(BundleError::UnexpectedEntry(name));
        }
    }

    let manifest_bytes =
        manifest_bytes.ok_or_else(|| BundleError::MissingEntry(MANIFEST_PATH.into()))?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)?;
    manifest.validate()?;
    ensure_correspondence(&manifest, evidence.keys())?;
    Ok(Bundle { manifest, evidence })
}

pub fn read_bundle_file(path: impl AsRef<Path>, limits: ReadLimits) -> Result<Bundle, BundleError> {
    read_bundle(File::open(path)?, limits)
}

impl Bundle {
    pub fn verify(&self) -> Result<(), BundleError> {
        for entry in &self.manifest.evidence {
            let bytes = self
                .evidence
                .get(&entry.path)
                .ok_or_else(|| BundleError::MissingEntry(entry.path.clone()))?;
            if bytes.len() as u64 != entry.size {
                return Err(BundleError::SizeMismatch(entry.path.clone()));
            }
            let digest = hex_digest(bytes);
            if digest != entry.sha256 {
                return Err(BundleError::HashMismatch(entry.path.clone()));
            }
        }
        Ok(())
    }

    pub fn extract(&self, destination: impl AsRef<Path>) -> Result<(), BundleError> {
        self.verify()?;
        let destination = destination.as_ref();
        fs::create_dir_all(destination)?;
        reject_symlink_or_special(destination)?;
        for (archive_path, bytes) in &self.evidence {
            let relative = archive_path
                .strip_prefix(EVIDENCE_PREFIX)
                .ok_or_else(|| BundleError::UnsafeDestination(archive_path.clone()))?;
            let output = destination.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
            let parent = output
                .parent()
                .ok_or_else(|| BundleError::UnsafeDestination(archive_path.clone()))?;
            fs::create_dir_all(parent)?;
            reject_path_components(destination, &output)?;
            if output.exists() {
                reject_symlink_or_special(&output)?;
            }
            let mut file = File::create(&output)?;
            file.write_all(bytes)?;
        }
        Ok(())
    }
}

pub fn verify_bundle_file(
    path: impl AsRef<Path>,
    limits: ReadLimits,
) -> Result<Bundle, BundleError> {
    let bundle = read_bundle_file(path, limits)?;
    bundle.verify()?;
    Ok(bundle)
}

fn ensure_correspondence<'a>(
    manifest: &Manifest,
    archive_paths: impl Iterator<Item = &'a String>,
) -> Result<(), BundleError> {
    let expected: HashSet<&str> = manifest
        .evidence
        .iter()
        .map(|entry| entry.path.as_str())
        .collect();
    let actual: HashSet<&str> = archive_paths.map(String::as_str).collect();
    if let Some(path) = expected.difference(&actual).next() {
        return Err(BundleError::MissingEntry((*path).to_string()));
    }
    if let Some(path) = actual.difference(&expected).next() {
        return Err(BundleError::UnexpectedEntry((*path).to_string()));
    }
    Ok(())
}

fn is_safe_archive_path(name: &str) -> bool {
    name == MANIFEST_PATH
        || (name.starts_with(EVIDENCE_PREFIX)
            && name.split('/').skip(1).all(|part| {
                !part.is_empty()
                    && part != "."
                    && part != ".."
                    && part.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'~')
                    })
            }))
}

fn is_special_file(entry: &zip::read::ZipFile<'_>) -> bool {
    entry.unix_mode().is_some_and(|mode| {
        let kind = mode & 0o170000;
        kind != 0 && kind != 0o100000
    })
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn reject_symlink_or_special(path: &Path) -> Result<(), BundleError> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_file() && !metadata.file_type().is_dir()
    {
        return Err(BundleError::UnsafeDestination(path.display().to_string()));
    }
    Ok(())
}

fn reject_path_components(destination: &Path, output: &Path) -> Result<(), BundleError> {
    let relative = output
        .strip_prefix(destination)
        .map_err(|_| BundleError::UnsafeDestination(output.display().to_string()))?;
    let mut current = destination.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if current.exists() {
            reject_symlink_or_special(&current)?;
        }
    }
    Ok(())
}

pub fn capture_mappings(
    manifest: &Manifest,
    mappings: &[(String, PathBuf)],
) -> Result<Vec<(String, Vec<u8>)>, BundleError> {
    let mut evidence = Vec::with_capacity(mappings.len());
    for (archive_path, source) in mappings {
        if !manifest
            .evidence
            .iter()
            .any(|entry| entry.path == *archive_path)
        {
            return Err(BundleError::UnexpectedEntry(archive_path.clone()));
        }
        evidence.push((archive_path.clone(), fs::read(source)?));
    }
    Ok(evidence)
}

pub fn hex_digest_for_bytes(bytes: &[u8]) -> String {
    hex_digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tempfile::tempdir;
    use zip::write::FileOptions;

    fn fixture_manifest() -> Manifest {
        serde_json::from_str(include_str!(
            "../conformance/fixtures/minimal-valid/manifest.json"
        ))
        .unwrap()
    }

    fn fixture_evidence() -> BTreeMap<String, Vec<u8>> {
        let mut evidence = BTreeMap::new();
        evidence.insert(
            "evidence/message.txt".into(),
            b"ReproPack minimal evidence.\n".to_vec(),
        );
        evidence
    }

    #[test]
    fn bundle_round_trip_and_verify() {
        let mut output = Cursor::new(Vec::new());
        output = create_bundle(output, &fixture_manifest(), &fixture_evidence()).unwrap();
        let bundle = read_bundle(Cursor::new(output.into_inner()), ReadLimits::default()).unwrap();
        bundle.verify().unwrap();
        assert_eq!(bundle.manifest, fixture_manifest());
    }

    #[test]
    fn missing_and_unexpected_entries_are_rejected() {
        let manifest = fixture_manifest();
        let empty = BTreeMap::new();
        assert!(matches!(
            create_bundle(Cursor::new(Vec::new()), &manifest, &empty),
            Err(BundleError::MissingEntry(_))
        ));
        let mut extra = fixture_evidence();
        extra.insert("evidence/extra.txt".into(), b"extra".to_vec());
        assert!(matches!(
            create_bundle(Cursor::new(Vec::new()), &manifest, &extra),
            Err(BundleError::UnexpectedEntry(_))
        ));
    }

    #[test]
    fn unsafe_and_directory_entries_are_rejected() {
        let mut bytes = Cursor::new(Vec::new());
        {
            let mut writer = ZipWriter::new(&mut bytes);
            let options = FileOptions::<()>::default();
            writer.start_file("../escape.txt", options).unwrap();
            writer.write_all(b"bad").unwrap();
            writer.finish().unwrap();
        }
        assert!(matches!(
            read_bundle(Cursor::new(bytes.into_inner()), ReadLimits::default()),
            Err(BundleError::UnsafePath(_))
        ));

        let mut directory = Cursor::new(Vec::new());
        {
            let mut writer = ZipWriter::new(&mut directory);
            writer
                .add_directory("evidence/dir", FileOptions::<()>::default())
                .unwrap();
            writer.finish().unwrap();
        }
        assert!(matches!(
            read_bundle(Cursor::new(directory.into_inner()), ReadLimits::default()),
            Err(BundleError::UnsupportedEntryType(_)) | Err(BundleError::UnsafePath(_))
        ));
    }

    #[test]
    fn limits_are_enforced_before_unbounded_read() {
        let mut output = Cursor::new(Vec::new());
        output = create_bundle(output, &fixture_manifest(), &fixture_evidence()).unwrap();
        let limits = ReadLimits {
            max_entry_bytes: 4,
            ..ReadLimits::default()
        };
        assert!(matches!(
            read_bundle(Cursor::new(output.into_inner()), limits),
            Err(BundleError::LimitExceeded(_))
        ));
    }

    #[test]
    fn malformed_archive_is_rejected_without_content_use() {
        assert!(matches!(
            read_bundle(Cursor::new(b"not a zip".to_vec()), ReadLimits::default()),
            Err(BundleError::Zip(_))
        ));
    }

    #[test]
    fn extraction_is_verified_and_stays_below_destination() {
        let mut output = Cursor::new(Vec::new());
        output = create_bundle(output, &fixture_manifest(), &fixture_evidence()).unwrap();
        let bundle = read_bundle(Cursor::new(output.into_inner()), ReadLimits::default()).unwrap();
        let destination = tempdir().unwrap();
        bundle.extract(destination.path()).unwrap();
        assert_eq!(
            fs::read(destination.path().join("message.txt")).unwrap(),
            b"ReproPack minimal evidence.\n"
        );
    }

    #[test]
    fn verification_rejects_changed_content() {
        let mut output = Cursor::new(Vec::new());
        output = create_bundle(output, &fixture_manifest(), &fixture_evidence()).unwrap();
        let mut bundle =
            read_bundle(Cursor::new(output.into_inner()), ReadLimits::default()).unwrap();
        bundle
            .evidence
            .insert("evidence/message.txt".into(), b"changed".to_vec());
        assert!(matches!(
            bundle.verify(),
            Err(BundleError::SizeMismatch(_)) | Err(BundleError::HashMismatch(_))
        ));
    }

    #[test]
    fn verification_rejects_same_size_content_with_wrong_hash() {
        let mut output = Cursor::new(Vec::new());
        output = create_bundle(output, &fixture_manifest(), &fixture_evidence()).unwrap();
        let mut bundle =
            read_bundle(Cursor::new(output.into_inner()), ReadLimits::default()).unwrap();
        bundle.evidence.insert(
            "evidence/message.txt".into(),
            b"XeproPack minimal evidence.\n".to_vec(),
        );
        assert!(matches!(bundle.verify(), Err(BundleError::HashMismatch(_))));
    }
}
