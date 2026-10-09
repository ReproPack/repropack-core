//! Conservative, explicit redaction helpers.
//!
//! Redaction is advisory and intentionally incomplete. These helpers cover a
//! small set of high-signal text forms; callers must still review evidence and
//! must not treat a clean scan as proof that no secret is present.

use crate::bundle::hex_digest_for_bytes;
use crate::{Manifest, Redaction, RedactionReason, RedactionStatus};
use thiserror::Error;

const MARKER: &str = "[REDACTED]";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactionWarning {
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactionResult {
    pub bytes: Vec<u8>,
    pub redacted: bool,
    pub warnings: Vec<RedactionWarning>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RedactionError {
    #[error("manifest evidence entry is missing: {0}")]
    MissingEvidence(String),
}

/// Redact high-signal secret forms from UTF-8 text.
pub fn redact_text(input: &str) -> RedactionResult {
    let mut output = String::with_capacity(input.len());
    let mut redacted = false;
    let mut warnings = Vec::new();
    let mut private_key = false;

    for line in input.split_inclusive('\n') {
        let (content, newline) = if let Some(content) = line.strip_suffix("\r\n") {
            (content, "\r\n")
        } else if let Some(content) = line.strip_suffix('\n') {
            (content, "\n")
        } else if let Some(content) = line.strip_suffix('\r') {
            (content, "\r")
        } else {
            (line, "")
        };

        if private_key {
            redacted = true;
            if content.contains("-----END ") && content.ends_with("PRIVATE KEY-----") {
                private_key = false;
            }
            output.push_str(MARKER);
            output.push_str(newline);
            continue;
        }

        if content.starts_with("-----BEGIN ") && content.ends_with("PRIVATE KEY-----") {
            private_key = true;
            redacted = true;
            warnings.push(warning(
                "private-key-block",
                "a private-key block was replaced without retaining its body",
            ));
            output.push_str(MARKER);
            output.push_str(newline);
            continue;
        }

        if let Some(index) = bearer_value_start(content) {
            redacted = true;
            warnings.push(warning("bearer-token", "a bearer-token value was replaced"));
            output.push_str(&content[..index]);
            output.push_str(MARKER);
            output.push_str(newline);
            continue;
        }

        if let Some((index, delimiter_end)) = secret_assignment(content) {
            redacted = true;
            warnings.push(warning(
                "secret-assignment",
                "a secret-like assignment value was replaced",
            ));
            output.push_str(&content[..delimiter_end]);
            output.push_str(MARKER);
            output.push_str(newline);
            let _ = index;
            continue;
        }

        output.push_str(content);
        output.push_str(newline);
    }

    RedactionResult {
        bytes: output.into_bytes(),
        redacted,
        warnings,
    }
}

/// Redact UTF-8 evidence, warning and preserving bytes when input is binary.
pub fn redact_bytes(input: &[u8]) -> RedactionResult {
    match std::str::from_utf8(input) {
        Ok(text) => redact_text(text),
        Err(_) => RedactionResult {
            bytes: input.to_vec(),
            redacted: false,
            warnings: vec![warning(
                "binary-input-not-scanned",
                "input is not valid UTF-8; no redaction was attempted",
            )],
        },
    }
}

/// Apply redaction to one manifest entry and return replacement evidence bytes.
///
/// When replacement occurs, the entry's size, digest, status, and reason are
/// updated together so a newly created bundle remains verifiable.
pub fn redact_manifest_entry(
    manifest: &mut Manifest,
    path: &str,
    input: &[u8],
    reason: RedactionReason,
) -> Result<RedactionResult, RedactionError> {
    let entry = manifest
        .evidence
        .iter_mut()
        .find(|entry| entry.path == path)
        .ok_or_else(|| RedactionError::MissingEvidence(path.to_owned()))?;
    let result = redact_bytes(input);
    if result.redacted {
        entry.size = result.bytes.len() as u64;
        entry.sha256 = hex_digest_for_bytes(&result.bytes);
        entry.redaction = Redaction {
            status: RedactionStatus::Redacted,
            reason: Some(reason),
        };
    }
    Ok(result)
}

fn warning(kind: &str, message: &str) -> RedactionWarning {
    RedactionWarning {
        kind: kind.to_owned(),
        message: message.to_owned(),
    }
}

fn bearer_value_start(line: &str) -> Option<usize> {
    let lower = line.to_ascii_lowercase();
    lower.find("bearer ").map(|index| index + "bearer ".len())
}

fn secret_assignment(line: &str) -> Option<(usize, usize)> {
    let delimiters = [':', '='];
    let delimiter = line
        .char_indices()
        .find(|(_, character)| delimiters.contains(character));
    let (delimiter_index, character) = delimiter?;
    let key = line[..delimiter_index].trim();
    let normalized = key
        .trim_start_matches(['-', '#', ' ', '\t'])
        .to_ascii_lowercase()
        .replace('-', "_");
    let secret_key = matches!(
        normalized.as_str(),
        "api_key"
            | "apikey"
            | "access_token"
            | "auth_token"
            | "password"
            | "secret"
            | "private_key"
            | "client_secret"
            | "authorization"
    );
    secret_key.then_some((delimiter_index, delimiter_index + character.len_utf8()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::{create_bundle, read_bundle, Bundle};
    use crate::Manifest;
    use std::collections::BTreeMap;
    use std::io::Cursor;

    fn manifest() -> Manifest {
        serde_json::from_str(include_str!(
            "../conformance/fixtures/minimal-valid/manifest.json"
        ))
        .unwrap()
    }

    #[test]
    fn redacts_secret_assignments_without_echoing_values() {
        let result = redact_text("password=super-secret\napi_key: abc123\n城市=東京\n");
        let text = String::from_utf8(result.bytes).unwrap();
        assert!(result.redacted);
        assert_eq!(text, "password=[REDACTED]\napi_key:[REDACTED]\n城市=東京\n");
        assert!(!text.contains("super-secret"));
        assert!(!text.contains("abc123"));
    }

    #[test]
    fn redacts_private_key_and_bearer_token() {
        let input = "-----BEGIN PRIVATE KEY-----\nsecret-body\n-----END PRIVATE KEY-----\nAuthorization: Bearer abc\n";
        let result = redact_text(input);
        let text = String::from_utf8(result.bytes).unwrap();
        assert!(result.redacted);
        assert!(!text.contains("secret-body"));
        assert!(!text.contains("Bearer abc"));
        assert!(text.contains(MARKER));
        assert_eq!(result.warnings.len(), 2);
    }

    #[test]
    fn redacts_crlf_private_key_without_retaining_body() {
        let result = redact_text(
            "-----BEGIN PRIVATE KEY-----\r\nsecret-body\r\n-----END PRIVATE KEY-----\r\n",
        );
        assert!(result.redacted);
        assert_eq!(result.bytes, b"[REDACTED]\r\n[REDACTED]\r\n[REDACTED]\r\n");
        assert!(!String::from_utf8_lossy(&result.bytes).contains("secret-body"));
    }

    #[test]
    fn binary_input_is_preserved_with_warning() {
        let input = b"\x00\xffsecret";
        let result = redact_bytes(input);
        assert_eq!(result.bytes, input);
        assert!(!result.redacted);
        assert_eq!(result.warnings[0].kind, "binary-input-not-scanned");
    }

    #[test]
    fn manifest_metadata_is_updated_and_bundle_verifies() {
        let mut manifest = manifest();
        let input = b"password=super-secret\n";
        let result = redact_manifest_entry(
            &mut manifest,
            "evidence/message.txt",
            input,
            RedactionReason::Secret,
        )
        .unwrap();
        assert!(result.redacted);
        assert_eq!(manifest.evidence[0].size, result.bytes.len() as u64);
        assert_eq!(
            manifest.evidence[0].sha256,
            hex_digest_for_bytes(&result.bytes)
        );
        assert_eq!(
            manifest.evidence[0].redaction.status,
            RedactionStatus::Redacted
        );
        let mut evidence = BTreeMap::new();
        evidence.insert("evidence/message.txt".to_owned(), result.bytes);
        let archive = create_bundle(Cursor::new(Vec::new()), &manifest, &evidence).unwrap();
        let bundle = read_bundle(Cursor::new(archive.into_inner()), Default::default()).unwrap();
        bundle.verify().unwrap();
        let _: Bundle = bundle;
    }

    #[test]
    fn missing_manifest_entry_is_reported() {
        let mut manifest = manifest();
        assert_eq!(
            redact_manifest_entry(
                &mut manifest,
                "evidence/missing.txt",
                b"x",
                RedactionReason::Secret
            ),
            Err(RedactionError::MissingEvidence(
                "evidence/missing.txt".into()
            ))
        );
    }
}
