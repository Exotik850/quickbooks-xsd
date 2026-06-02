#![doc = include_str!("../README.md")]
#![allow(dead_code, unused_variables, unused_mut)]

pub use xsd_parser_types as types;

#[cfg(feature = "generate")]
include!(concat!(env!("OUT_DIR"), "/schemas.rs"));

#[cfg(not(feature = "generate"))]
mod schemas;
#[cfg(not(feature = "generate"))]
pub use schemas::*;