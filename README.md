# quickbooks-xsd

A Rust crate providing strongly-typed bindings for the QuickBooks API generated from their official XSD schemas.

## Overview

Due to licensing and redistribution restrictions on the original QuickBooks XSD files, this crate does not bundle the schema files directly. Instead, when the `generate` feature is enabled, the build script dynamically downloads the official QuickBooks XSD zip file corresponding to the crate's current MINOR semver version (e.g., `v3-minor-version-75` for `3.75.x`). It then utilizes `xsd-parser` to parse the downloaded XSD and generate the required Rust schemas and structs at build time. 

The generated code is still legally usable and is included in the crates.io package, but the original XSD files are not redistributed. This approach allows users to benefit from strongly-typed bindings while respecting the licensing constraints of the original schemas.

## Usage

Add the crate to your `Cargo.toml`. 

```toml
[dependencies]
quickbooks-xsd = { version = "3.75.0" }
# To download and generate the schemas over the network during compilation (very slow):
quickbooks-xsd = { version = "3.75.0", features = ["generate"] }
```

### Example

Here is a brief example demonstrating how to parse a QuickBooks invoice XML payload into the strongly-typed generated structs:

```rust
use quickbooks_xsd::finance::InvoiceType;
use xsd_parser_types::quick_xml::DeserializeSync;
use std::fs::OpenOptions;

fn main() {
    let xml_file = OpenOptions::new()
        .read(true)
        .open("./examples/invoice.xml")
        .unwrap();
        
    let reader = std::io::BufReader::new(xml_file);
    let mut xml_reader = xsd_parser_types::quick_xml::reader::IoReader::new(reader).with_error_info();
    
    let parsed_invoice = InvoiceType::deserialize(&mut xml_reader).unwrap();
    println!("{:#?}", parsed_invoice);
}
```

You can find more usage scenarios in the `examples/` directory.

## License

This project is licensed under the [MIT License](LICENSE).
