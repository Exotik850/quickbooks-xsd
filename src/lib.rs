//! # quickbooks-xsd
//!
//! XSD Types for the Quickbooks QBO API generated for Rust

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub use xsd_parser_types as types;

mod schemas;
pub use schemas::*;