use quickbooks_xsd::finance::InvoiceType;
// use serde::Deserialize;
use xsd_parser_types::quick_xml::{self, DeserializeSync, XmlReader};

fn main() {
    // let xml = include_str!("./invoice.xml");
    let xml = std::fs::OpenOptions::new()
        .read(true)
        .open("./examples/invoice.xml")
        .unwrap();
    let reader = std::io::BufReader::new(xml);
    let mut reader = quick_xml::reader::IoReader::new(reader).with_error_info();
    // let mut reader = quick_xml::reader::SliceReader::new(xml);
    let parsed = InvoiceType::deserialize(&mut reader).unwrap();
    println!("{:#?}", parsed);
}
