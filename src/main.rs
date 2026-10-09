use repropack_core::bundle::{
    capture_mappings, create_bundle, read_bundle_file, verify_bundle_file, ReadLimits,
};
use repropack_core::Manifest;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;

fn usage() -> &'static str {
    "Usage:\n  repropack capture <manifest.json> <output.rpk> <archive_path=source_path>...\n  repropack inspect <bundle.rpk>\n  repropack validate <bundle.rpk>\n  repropack verify <bundle.rpk>\n  repropack extract <bundle.rpk> <destination>"
}

fn main() {
    if let Err(error) = run(std::env::args().skip(1).collect()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let Some(command) = args.first().map(String::as_str) else {
        println!("{}", usage());
        return Ok(());
    };
    match command {
        "capture" => {
            if args.len() < 4 {
                return Err(usage().into());
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
        }
        "inspect" => {
            let bundle = read_bundle_file(required(&args, 1)?, ReadLimits::default())?;
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
        "validate" => {
            read_bundle_file(required(&args, 1)?, ReadLimits::default())?;
            println!("valid");
        }
        "verify" => {
            verify_bundle_file(required(&args, 1)?, ReadLimits::default())?;
            println!("verified");
        }
        "extract" => {
            let bundle = read_bundle_file(required(&args, 1)?, ReadLimits::default())?;
            bundle.extract(required(&args, 2)?)?;
            println!("extracted");
        }
        _ => return Err(usage().into()),
    }
    Ok(())
}

fn required(args: &[String], index: usize) -> Result<&str, io::Error> {
    args.get(index)
        .map(String::as_str)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, usage()))
}
