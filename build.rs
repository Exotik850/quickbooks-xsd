

#[cfg(feature = "generate")]
mod generate;

fn main() {
    #[cfg(feature = "generate")]
    generate::build_schema_file();
}
