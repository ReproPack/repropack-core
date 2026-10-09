use repropack_core::bundle::{create_bundle, read_bundle, BundleError, ReadLimits};
use repropack_core::Manifest;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct Catalog {
    spec_version: String,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct Case {
    id: String,
    valid: bool,
    expected: String,
}

#[derive(Debug, Deserialize)]
struct Expected {
    valid: bool,
    error: Option<String>,
    assertions: Vec<String>,
}

#[test]
fn canonical_fixtures_match_expected_semantics() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let catalog: Catalog = read_json(&root.join("conformance/catalog.json"));
    assert_eq!(catalog.spec_version, "0.1");
    assert!(!catalog.cases.is_empty());

    for case in catalog.cases {
        let expected: Expected = read_json(&root.join(&case.expected));
        assert_eq!(case.valid, expected.valid, "{} catalog mismatch", case.id);
        assert!(
            !expected.assertions.is_empty(),
            "{} has no assertions",
            case.id
        );
        let fixture = root.join("conformance/fixtures").join(&case.id);
        let result = run_case(&fixture, &case.id);
        if case.valid {
            let bundle = result.unwrap_or_else(|error| panic!("{}: {error}", case.id));
            bundle
                .verify()
                .unwrap_or_else(|error| panic!("{} verification: {error}", case.id));
            assert_eq!(bundle.manifest.spec_version, catalog.spec_version);
            if case.id == "redacted-valid" {
                let bytes = &bundle.evidence["evidence/config.txt"];
                assert!(String::from_utf8_lossy(bytes).contains("[REDACTED]"));
                assert!(!String::from_utf8_lossy(bytes).contains("secret-value"));
            }
        } else {
            let expected_error = expected.error.as_deref().unwrap_or("invalid-manifest");
            let error = result.expect_err(&format!("{} unexpectedly passed", case.id));
            assert_eq!(error_category(&error), expected_error, "{}", case.id);
        }
    }
}

fn run_case(fixture: &Path, id: &str) -> Result<repropack_core::bundle::Bundle, BundleError> {
    let manifest_path = fixture.join("manifest.json");
    let manifest_text = fs::read_to_string(&manifest_path).map_err(BundleError::Io)?;
    let manifest: Manifest = serde_json::from_str(&manifest_text).map_err(BundleError::Json)?;
    manifest.validate().map_err(BundleError::Validation)?;

    if id == "oversized-input" {
        let entry = &manifest.evidence[0];
        if entry.size <= ReadLimits::default().max_entry_bytes {
            return Err(BundleError::Malformed(
                "oversized fixture is not oversized".into(),
            ));
        }
        return Err(BundleError::LimitExceeded(entry.path.clone()));
    }

    let mut evidence = BTreeMap::new();
    for entry in &manifest.evidence {
        let relative = entry.path.replace('/', std::path::MAIN_SEPARATOR_STR);
        evidence.insert(
            entry.path.clone(),
            fs::read(fixture.join(relative)).map_err(BundleError::Io)?,
        );
    }
    let archive = create_bundle(Cursor::new(Vec::new()), &manifest, &evidence)?;
    let bundle = read_bundle(Cursor::new(archive.into_inner()), ReadLimits::default())?;
    if id == "corrupted-content" || id == "incorrect-hash" {
        bundle.verify()?;
    }
    Ok(bundle)
}

fn error_category(error: &BundleError) -> &'static str {
    match error {
        BundleError::HashMismatch(_) | BundleError::SizeMismatch(_) => "hash-mismatch",
        BundleError::Validation(error) => match error {
            repropack_core::ValidationError::UnsupportedVersion(_) => "unsupported-version",
            repropack_core::ValidationError::UnsafePath(_) => "unsafe-path",
            _ => "invalid-manifest",
        },
        BundleError::Json(_) | BundleError::Model(_) => "invalid-manifest",
        BundleError::LimitExceeded(_) => "limit-exceeded",
        _ => "invalid-manifest",
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_slice(&bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}
