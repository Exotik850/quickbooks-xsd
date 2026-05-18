//! # quickbooks-xsd
//!
//! XSD Types for the Quickbooks QBO API generated for Rust

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(nonstandard_style)]

#[cfg(feature = "generate")]
include!(concat!(env!("OUT_DIR"), "/schemas.rs"));

#[cfg(not(feature = "generate"))]
include!("schemas.rs");
