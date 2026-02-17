// TODO: Documentation

pub mod build;
pub mod config;
pub mod lexer;
pub mod modification;
pub mod parser;

#[macro_use]
extern crate unwrap;

// #[macro_use]
// extern crate enum_dispatch;

// Localization
rust_i18n::i18n!("i18n", fallback = "en");
