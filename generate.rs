use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
};
use xsd_parser::{
    Config,
    config::{
        GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Schema, SerdeXmlRsVersion,
    },
    models::ExplicitNaming,
};

const LATEST_ZIP_URL: &str =
    "https://static.developer.intuit.com/resources/v3-minor-version-75.zip";
const LATEST_VERSION: &str = "v3-minor-version-75";

pub(crate) fn build_schema_file() {
    let out_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();

    // download the zip and extract it to a temporary directory
    let response = ureq::get(LATEST_ZIP_URL)
        .call()
        .expect("Could not download XSD zip file");
    let cursor = std::io::Cursor::new(
        response
            .into_body()
            .read_to_vec()
            .expect("Could not read XSD zip file into memory"),
    );
    let mut zip = zip::ZipArchive::new(cursor).expect("Could not read XSD zip file");
    let extract_dir = out_dir.join("extracted");
    std::fs::create_dir_all(&extract_dir)
        .expect("Could not create directory for extracted XSD files");
    zip.extract(&extract_dir)
        .expect("Could not extract XSD zip file");

    let xsd_files = std::fs::read_dir(&extract_dir.join(LATEST_VERSION))
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
        .with_schemas(xsd_files.into_iter().map(Schema::File)) //TODO: Dedupe imported schemas
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

            // TODO: This makes larger structs like `InvoiceType` easier to read / handle,
            // but having it enabled also causes stack overflows during deserialization because of the 
            // large number of fields. upstream issue?
            // ^ OptimizerFlags::FLATTEN_COMPLEX_TYPES 
            
                ^ OptimizerFlags::REPLACE_XS_ANY_TYPE_WITH_ANY_ELEMENT,
        )
        .with_quick_xml();
    let code = xsd_parser::generate(config).expect("Could not generate schemas");
    let code = code.to_string();
    let code = rustfmt_pretty_print(code).expect("Could not format generated code");
    std::fs::write(out_dir.join("src/schemas.rs"), code)
        .expect("Could not write generated code to file");
}

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
