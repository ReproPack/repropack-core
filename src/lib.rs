//! ReproPack 0.1 typed manifest model.
//!
//! This crate intentionally stops at the Phase 2 data model boundary. It does
//! not read ZIP archives, extract files, or execute bundle contents.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use thiserror::Error;

pub mod bundle;

pub const FORMAT: &str = "repropack";
pub const SPEC_VERSION: &str = "0.1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: String,
    pub spec_version: String,
    pub bundle_id: String,
    pub created_at: String,
    pub capture: Capture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incident: Option<Incident>,
    pub evidence: Vec<EvidenceEntry>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Capture {
    pub mode: CaptureMode,
    pub tool: Tool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CaptureMode {
    Explicit,
    Generated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Incident {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reported_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceEntry {
    pub path: String,
    pub kind: EvidenceKind,
    pub media_type: String,
    pub size: u64,
    pub sha256: String,
    pub selection: Selection,
    pub redaction: Redaction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceKind {
    Log,
    Text,
    Structured,
    Source,
    Environment,
    TestOutput,
    File,
}

impl EvidenceKind {
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Log => "log",
            Self::Text => "text",
            Self::Structured => "structured",
            Self::Source => "source",
            Self::Environment => "environment",
            Self::TestOutput => "test-output",
            Self::File => "file",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Selection {
    Explicit,
    Generated,
    Derived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Redaction {
    pub status: RedactionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<RedactionReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RedactionStatus {
    None,
    Redacted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum RedactionReason {
    Secret,
    PersonalData,
    UserRequested,
    Policy,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("format must be {FORMAT}")]
    InvalidFormat,
    #[error("unsupported specification version: {0}")]
    UnsupportedVersion(String),
    #[error("bundle_id must be a lowercase RFC 4122 UUID")]
    InvalidBundleId,
    #[error("invalid UTC timestamp at {0}")]
    InvalidTimestamp(String),
    #[error("capture tool name and version are required")]
    InvalidTool,
    #[error("extension key is not an absolute URI: {0}")]
    InvalidExtension(String),
    #[error("evidence must contain at least one entry")]
    EmptyEvidence,
    #[error("evidence paths must be sorted lexicographically")]
    UnsortedEvidence,
    #[error("duplicate evidence path: {0}")]
    DuplicateEvidencePath(String),
    #[error("unsafe evidence path: {0}")]
    UnsafePath(String),
    #[error("invalid media type: {0}")]
    InvalidMediaType(String),
    #[error("invalid SHA-256 digest at {0}")]
    InvalidDigest(String),
    #[error("invalid redaction at {0}")]
    InvalidRedaction(String),
    #[error("invalid incident timestamp")]
    InvalidIncidentTimestamp,
}

#[derive(Debug, Error)]
pub enum ModelError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl Manifest {
    /// Validate model rules that do not require an archive.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.format != FORMAT {
            return Err(ValidationError::InvalidFormat);
        }
        if self.spec_version != SPEC_VERSION {
            return Err(ValidationError::UnsupportedVersion(
                self.spec_version.clone(),
            ));
        }
        if !is_uuid(&self.bundle_id) {
            return Err(ValidationError::InvalidBundleId);
        }
        if !is_utc_timestamp(&self.created_at) {
            return Err(ValidationError::InvalidTimestamp("created_at".into()));
        }
        if self.capture.tool.name.is_empty() || self.capture.tool.version.is_empty() {
            return Err(ValidationError::InvalidTool);
        }
        for key in self.extensions.keys() {
            if !is_absolute_uri(key) {
                return Err(ValidationError::InvalidExtension(key.clone()));
            }
        }
        if let Some(incident) = &self.incident {
            if incident
                .reported_at
                .as_deref()
                .is_some_and(|value| !is_utc_timestamp(value))
            {
                return Err(ValidationError::InvalidIncidentTimestamp);
            }
        }
        if self.evidence.is_empty() {
            return Err(ValidationError::EmptyEvidence);
        }
        let mut paths = HashSet::new();
        let mut previous: Option<&str> = None;
        for entry in &self.evidence {
            if !is_safe_evidence_path(&entry.path) {
                return Err(ValidationError::UnsafePath(entry.path.clone()));
            }
            if !paths.insert(&entry.path) {
                return Err(ValidationError::DuplicateEvidencePath(entry.path.clone()));
            }
            if previous.is_some_and(|path| entry.path.as_str() <= path) {
                return Err(ValidationError::UnsortedEvidence);
            }
            previous = Some(&entry.path);
            if !is_media_type(&entry.media_type) {
                return Err(ValidationError::InvalidMediaType(entry.media_type.clone()));
            }
            if !is_digest(&entry.sha256) {
                return Err(ValidationError::InvalidDigest(entry.path.clone()));
            }
            match (&entry.redaction.status, &entry.redaction.reason) {
                (RedactionStatus::None, Some(_)) | (RedactionStatus::Redacted, None) => {
                    return Err(ValidationError::InvalidRedaction(entry.path.clone()))
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Serialize a validated manifest as compact deterministic JSON.
    pub fn to_canonical_json(&self) -> Result<String, ModelError> {
        self.validate()?;
        Ok(serde_json::to_string(&serde_json::to_value(self)?)?)
    }
}

fn is_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && [8, 13, 18, 23].iter().all(|&index| bytes[index] == b'-')
        && bytes.iter().enumerate().all(|(index, byte)| {
            [8, 13, 18, 23].contains(&index)
                || byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()
        })
        && matches!(bytes[14], b'1'..=b'5')
        && matches!(bytes[19], b'8' | b'9' | b'a' | b'b')
}

fn is_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !value.is_ascii() || !value.ends_with('Z') || bytes.len() < 20 || bytes.len() > 30 {
        return false;
    }
    let (base, fraction) = match value[..value.len() - 1].split_once('.') {
        Some((base, fraction)) => (base, Some(fraction)),
        None => (&value[..value.len() - 1], None),
    };
    if base.len() != 19
        || ![4, 7]
            .iter()
            .all(|&i| base.as_bytes().get(i) == Some(&b'-'))
        || base.as_bytes().get(10) != Some(&b'T')
        || ![13, 16]
            .iter()
            .all(|&i| base.as_bytes().get(i) == Some(&b':'))
        || !base
            .as_bytes()
            .iter()
            .enumerate()
            .all(|(i, byte)| matches!(i, 4 | 7 | 10 | 13 | 16) || byte.is_ascii_digit())
        || !fraction.is_none_or(|digits| {
            (1..=9).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return false;
    }
    let date = base.as_bytes();
    let month = two_digits(&date[5..7]);
    let day = two_digits(&date[8..10]);
    let hour = two_digits(&date[11..13]);
    let minute = two_digits(&date[14..16]);
    let second = two_digits(&date[17..19]);
    let Some((month, day, hour, minute, second)) = month
        .zip(day)
        .zip(hour)
        .zip(minute)
        .zip(second)
        .map(|((((month, day), hour), minute), second)| (month, day, hour, minute, second))
    else {
        return false;
    };
    let year = u32::from(date[0] - b'0') * 1000
        + u32::from(date[1] - b'0') * 100
        + u32::from(date[2] - b'0') * 10
        + u32::from(date[3] - b'0');
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };
    (1..=days_in_month).contains(&day) && month <= 12 && hour < 24 && minute < 60 && second < 60
}

fn two_digits(value: &[u8]) -> Option<u32> {
    if value.len() == 2 && value[0].is_ascii_digit() && value[1].is_ascii_digit() {
        Some(u32::from(value[0] - b'0') * 10 + u32::from(value[1] - b'0'))
    } else {
        None
    }
}

fn is_absolute_uri(value: &str) -> bool {
    value.contains(':') && !value.chars().any(char::is_whitespace)
}

fn is_safe_evidence_path(value: &str) -> bool {
    value.starts_with("evidence/")
        && value.split('/').skip(1).all(|component| {
            !component.is_empty()
                && component != "."
                && component != ".."
                && component.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'~')
                })
        })
}

fn is_media_type(value: &str) -> bool {
    let mut parts = value.split('/');
    let (Some(type_part), Some(subtype), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    !type_part.is_empty()
        && !subtype.is_empty()
        && value == value.to_ascii_lowercase()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(
                    byte,
                    b'!' | b'#'
                        | b'$'
                        | b'%'
                        | b'&'
                        | b'\''
                        | b'*'
                        | b'+'
                        | b'.'
                        | b'^'
                        | b'_'
                        | b'`'
                        | b'|'
                        | b'~'
                        | b'-'
                        | b'/'
                )
        })
}

fn is_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Manifest {
        let path = format!(
            "{}/conformance/fixtures/{name}/manifest.json",
            env!("CARGO_MANIFEST_DIR")
        );
        serde_json::from_str(&std::fs::read_to_string(path).expect("fixture")).expect("manifest")
    }

    #[test]
    fn valid_fixtures_validate() {
        for name in [
            "minimal-valid",
            "complete-valid",
            "redacted-valid",
            "selected-evidence-valid",
            "unknown-extension-valid",
        ] {
            fixture(name).validate().expect(name);
        }
    }

    #[test]
    fn invalid_metadata_and_paths_are_rejected() {
        let invalid = std::fs::read_to_string(format!(
            "{}/conformance/fixtures/invalid-metadata/manifest.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        assert!(serde_json::from_str::<Manifest>(&invalid).is_err());
        let unsafe_manifest = fixture("unsafe-path");
        assert!(matches!(
            unsafe_manifest.validate(),
            Err(ValidationError::UnsafePath(_))
        ));
    }

    #[test]
    fn unknown_fields_are_rejected_during_deserialization() {
        let result = serde_json::from_str::<Manifest>(
            r#"{"format":"repropack","spec_version":"0.1","bundle_id":"11111111-1111-4111-8111-111111111111","created_at":"2026-01-01T00:00:00Z","capture":{"mode":"explicit","tool":{"name":"x","version":"1"}},"evidence":[],"typo":true}"#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn canonical_json_is_compact_and_validated() {
        let json = fixture("minimal-valid")
            .to_canonical_json()
            .expect("canonical JSON");
        assert!(!json.contains('\n'));
        assert!(json.starts_with("{\"bundle_id\""));
        assert!(serde_json::from_str::<Value>(&json).is_ok());
    }

    #[test]
    fn unsupported_version_is_rejected() {
        let mut manifest = fixture("minimal-valid");
        manifest.spec_version = "9.0".into();
        assert!(matches!(
            manifest.validate(),
            Err(ValidationError::UnsupportedVersion(_))
        ));
    }

    #[test]
    fn semantic_constraints_reject_duplicate_unsorted_and_invalid_entries() {
        let mut duplicate = fixture("minimal-valid");
        duplicate.evidence.push(duplicate.evidence[0].clone());
        assert!(matches!(
            duplicate.validate(),
            Err(ValidationError::DuplicateEvidencePath(_))
        ));

        let mut unsorted = fixture("complete-valid");
        unsorted.evidence.swap(0, 1);
        assert!(matches!(
            unsorted.validate(),
            Err(ValidationError::UnsortedEvidence)
        ));

        let mut bad_digest = fixture("minimal-valid");
        bad_digest.evidence[0].sha256 = "x".into();
        assert!(matches!(
            bad_digest.validate(),
            Err(ValidationError::InvalidDigest(_))
        ));

        let mut bad_redaction = fixture("minimal-valid");
        bad_redaction.evidence[0].redaction.reason = Some(RedactionReason::Secret);
        assert!(matches!(
            bad_redaction.validate(),
            Err(ValidationError::InvalidRedaction(_))
        ));
    }

    #[test]
    fn timestamp_media_type_and_extension_rules_are_enforced() {
        let mut manifest = fixture("minimal-valid");
        manifest.created_at = "2026-02-29T00:00:00Z".into();
        assert!(matches!(
            manifest.validate(),
            Err(ValidationError::InvalidTimestamp(_))
        ));

        let mut media = fixture("minimal-valid");
        media.evidence[0].media_type = "TEXT/PLAIN".into();
        assert!(matches!(
            media.validate(),
            Err(ValidationError::InvalidMediaType(_))
        ));

        let mut extension = fixture("minimal-valid");
        extension.extensions.insert("not a uri".into(), Value::Null);
        assert!(matches!(
            extension.validate(),
            Err(ValidationError::InvalidExtension(_))
        ));
    }

    #[test]
    fn json_round_trip_preserves_complete_model() {
        let manifest = fixture("complete-valid");
        let json = manifest.to_canonical_json().expect("canonical JSON");
        let round_trip: Manifest = serde_json::from_str(&json).expect("round trip");
        assert_eq!(manifest, round_trip);
    }
}
