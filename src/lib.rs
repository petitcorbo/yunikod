pub mod entities;
pub mod items;
pub mod inventory;
pub mod game;
pub mod blocks;
pub mod chunk;
pub mod ui;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");