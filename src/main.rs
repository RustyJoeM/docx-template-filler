#![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod lang;
use lang::SupportedLanguage;

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

mod docx_filler;
mod ui;

fn main() {
    lang::set_current_lang(SupportedLanguage::EnglishUs);
    ui::init_app();
}
