use repropack_core::bundle::{
    capture_mappings, create_bundle, read_bundle_file, verify_bundle_file, BundleError, ReadLimits,
};
use repropack_core::{Manifest, ModelError, ValidationError};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;

fn usage() -> &'static str {
    "Usage:\n  repropack [--json] capture <manifest.json> <output.rpk> <archive_path=source_path>...\n  repropack [--json] inspect <bundle.rpk>\n  repropack [--json] validate <bundle.rpk>\n  repropack [--json] verify <bundle.rpk>\n  repropack [--json] extract <bundle.rpk> <destination>\n\nExit codes:\n  0 success  2 usage error  3 input/validation error  4 internal error"
}

#[derive(Debug)]
struct UsageError(String);

impl std::fmt::Display for UsageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for UsageError {}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let json_mode = args.first().is_some_and(|arg| arg == "--json");
    if json_mode {
        args.remove(0);
    }
    if let Err(error) = run(args, json_mode) {
        let (code, message, exit) = classify_error(error.as_ref());
        if json_mode {
            println!(
                "{}",
                serde_json::json!({"error": {"code": code, "message": message}})
            );
        } else {
            eprintln!("error[{code}]: {message}");
        }
        std::process::exit(exit);
    }
}

fn run(args: Vec<String>, json_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let Some(command) = args.first().map(String::as_str) else {
        if json_mode {
            println!("{}", serde_json::json!({"usage": usage()}));
        } else {
            println!("{}", usage());
        }
        return Ok(());
    };
    match command {
        "capture" => {
            if args.len() < 4 {
                return Err(UsageError(usage().into()).into());
            }
            let manifest: Manifest = serde_json::from_reader(File::open(&args[1])?)?;
            let mappings = args[3..]
                .iter()
                .map(|mapping| {
                    let (archive_path, source_path) = mapping
                        .split_once('=')
                        .ok_or_else(|| format!("invalid mapping: {mapping}"))?;
                    Ok((archive_path.to_string(), PathBuf::from(source_path)))
                })
                .collect::<Result<Vec<_>, String>>()?;
            let mapped = capture_mappings(&manifest, &mappings)?;
            let evidence = mapped.into_iter().collect::<BTreeMap<_, _>>();
            let output = File::create(&args[2])?;
            let _ = create_bundle(output, &manifest, &evidence)?;
            if json_mode {
                println!(
                    "{}",
                    serde_json::json!({"status": "created", "path": &args[2]})
                );
            }
        }
        "inspect" => {
            let bundle = read_bundle_file(required(&args, 1)?, ReadLimits::default())?;
            if json_mode {
                println!(
                    "{}",
                    serde_json::json!({
                        "format": bundle.manifest.format,
                        "spec_version": bundle.manifest.spec_version,
                        "bundle_id": bundle.manifest.bundle_id,
                        "evidence": bundle.manifest.evidence.iter().map(|entry| serde_json::json!({"path": entry.path, "size": entry.size, "kind": entry.kind.kind_name()})).collect::<Vec<_>>()
                    })
                );
            } else {
                println!(
                    "format={} spec_version={} bundle_id={}",
                    bundle.manifest.format, bundle.manifest.spec_version, bundle.manifest.bundle_id
                );
                for entry in &bundle.manifest.evidence {
                    println!(
                        "{} {} bytes {}",
                        entry.path,
                        entry.size,
                        entry.kind.kind_name()
                    );
                }
            }
        }
        "validate" => {
            read_bundle_file(required(&args, 1)?, ReadLimits::default())?;
            if json_mode {
                println!("{{\"status\":\"valid\"}}");
            } else {
                println!("valid");
            }
        }
        "verify" => {
            verify_bundle_file(required(&args, 1)?, ReadLimits::default())?;
            if json_mode {
                println!("{{\"status\":\"verified\"}}");
            } else {
                println!("verified");
            }
        }
        "extract" => {
            let bundle = read_bundle_file(required(&args, 1)?, ReadLimits::default())?;
            bundle.extract(required(&args, 2)?)?;
            if json_mode {
                println!("{{\"status\":\"extracted\"}}");
            } else {
                println!("extracted");
            }
        }
        _ => return Err(UsageError(usage().into()).into()),
    }
    Ok(())
}

fn classify_error(error: &(dyn Error + 'static)) -> (&'static str, String, i32) {
    if let Some(error) = error.downcast_ref::<BundleError>() {
        let code = match error {
            BundleError::UnsafePath(_) | BundleError::UnsafeDestination(_) => "unsafe-path",
            BundleError::DuplicateEntry(_) => "duplicate-entry",
            BundleError::EncryptedEntry(_) => "encrypted-entry",
            BundleError::UnsupportedEntryType(_) => "unsupported-entry-type",
            BundleError::LimitExceeded(_) => "limit-exceeded",
            BundleError::MissingEntry(_) => "missing-entry",
            BundleError::UnexpectedEntry(_) => "unexpected-entry",
            BundleError::HashMismatch(_) => "hash-mismatch",
            BundleError::SizeMismatch(_) => "size-mismatch",
            BundleError::Validation(ValidationError::UnsupportedVersion(_)) => {
                "unsupported-version"
            }
            BundleError::Validation(_) | BundleError::Json(_) | BundleError::Model(_) => {
                "invalid-manifest"
            }
            BundleError::Zip(_) | BundleError::Malformed(_) => "malformed-archive",
            BundleError::Io(_) | BundleError::InvalidCaptureMapping(_) => "input-error",
        };
        return (code, error.to_string(), 3);
    }
    if error.downcast_ref::<ModelError>().is_some()
        || error.downcast_ref::<serde_json::Error>().is_some()
    {
        return ("invalid-manifest", error.to_string(), 3);
    }
    if let Some(error) = error.downcast_ref::<std::io::Error>() {
        if error.kind() == std::io::ErrorKind::InvalidInput {
            return ("usage-error", error.to_string(), 2);
        }
        return ("input-error", error.to_string(), 3);
    }
    if error.downcast_ref::<UsageError>().is_some() {
        return ("usage-error", error.to_string(), 2);
    }
    ("internal-error", error.to_string(), 4)
}

fn required(args: &[String], index: usize) -> Result<&str, io::Error> {
    args.get(index)
        .map(String::as_str)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, usage()))
}
