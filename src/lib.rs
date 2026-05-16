//! # quickbooks-xsd
//! 
//! XSD Types for the Quickbooks QBO API generated for Rust
//! 

#[cfg(feature = "generate")]
include!(concat!(env!("OUT_DIR"), "/schemas.rs"));

#[cfg(not(feature = "generate"))]
mod schemas;