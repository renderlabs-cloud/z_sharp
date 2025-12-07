// TODO: Documentation


pub mod build;
pub mod modification;
pub mod config;

#[macro_use]
extern crate rust_i18n;

#[macro_use]
extern crate unwrap;

pub use crate::build::builder;

// Localization
i18n!("i18n", fallback = "en");