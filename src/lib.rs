#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindgen;
mod inline_fns;

pub use bindgen::*;
pub use inline_fns::*;
#[link(name = "lilv-0")]
extern "C" {}
