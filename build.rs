#[cfg(feature = "generate")]
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
};
#[cfg(feature = "generate")]
use xsd_parser::{
    Config,
    config::{
        GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Schema, SerdeXmlRsVersion,
    },
    models::ExplicitNaming,
};

fn main() {
    #[cfg(feature = "generate")]
    build_schema_file();
}

#[cfg(feature = "generate")]
fn build_schema_file() {
    let cargo_manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let xsd_dir = cargo_manifest_dir.join("xsd");
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();

    println!("cargo:rerun-if-changed={}", xsd_dir.display());

    // xsd has folders for the different versions of the schema,
    // the name of the folder is the version, so find the latest version

    let latest_version = std::fs::read_dir(&xsd_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                Some((
                    entry
                        .file_name()
                        .to_string_lossy()
                        .parse::<u32>()
                        .unwrap_or(0),
                    entry.path(),
                ))
            } else {
                None
            }
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(version, path)| {
            println!("Found version: {version}");
            path
        })
        .unwrap();

    let xsd_files = std::fs::read_dir(&latest_version)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() && entry.path().extension().unwrap() == "xsd" {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("Found XSD files: {}", xsd_files.len());

    let config = Config::default()
        .with_schemas(xsd_files.into_iter().map(Schema::File))
        .with_generator_flags(
            GeneratorFlags::all()
                ^ GeneratorFlags::ANY_TYPE_SUPPORT
                ^ GeneratorFlags::ADVANCED_ENUMS,
        )
        .with_naming(ExplicitNaming::default())
        .with_interpreter_flags(
            InterpreterFlags::all()
                ^ InterpreterFlags::WITH_NUM_BIG_INT
                ^ InterpreterFlags::WITH_XS_ANY_TYPE
                ^ InterpreterFlags::WITH_XS_ANY_SIMPLE_TYPE,
        )
        .with_parser_flags(ParserFlags::all())
        .with_optimizer_flags(
            OptimizerFlags::all()
                // ^ OptimizerFlags::FLATTEN_COMPLEX_TYPES
                ^ OptimizerFlags::REPLACE_XS_ANY_TYPE_WITH_ANY_ELEMENT,
        )
        // .with_derive(["serde::Serialize", "serde::Deserialize", "Debug"])
        // .with_serde_quick_xml()
        .with_quick_xml();
    let code = xsd_parser::generate(config).expect("Could not generate schemas");
    let code = code.to_string();
    let code = rustfmt_pretty_print(code).expect("Could not format generated code");
    std::fs::write(out_dir.join("schemas.rs"), code)
        .expect("Could not write generated code to file");
}

#[cfg(feature = "generate")]
// A small helper to call `rustfmt` when generating file(s).
// This may be useful to compare different versions of generated files.
pub fn rustfmt_pretty_print(code: String) -> Result<String, xsd_parser::Error> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    write!(stdin, "{code}")?;
    stdin.flush()?;
    drop(stdin);

    let Output {
        status,
        stdout,
        stderr,
    } = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);

    if !status.success() {
        let code = status.code();
        match code {
            Some(code) => {
                assert!(
                    code == 0,
                    "The `rustfmt` command failed with return code {code}!\n{stderr}"
                );
            }
            None => {
                panic!("The `rustfmt` command failed!\n{stderr}")
            }
        }
    }

    Ok(stdout.into())
}
