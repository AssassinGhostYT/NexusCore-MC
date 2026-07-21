// Sistema de idiomas global para NexusCore-MC
pub mod es;
pub mod en;

use std::sync::atomic::{AtomicU8, Ordering};

static LANG: AtomicU8 = AtomicU8::new(0);

pub fn set_language(lang: u8) {
    LANG.store(lang, Ordering::Relaxed);
}

pub fn get_language() -> u8 {
    LANG.load(Ordering::Relaxed)
}

/// Macro para obtener el string correcto segun el idioma
#[macro_export]
macro_rules! t {
    ($field:ident) => {
        match $crate::lang::get_language() {
            1 => $crate::lang::en::En::$field,
            _ => $crate::lang::es::Es::$field,
        }
    };
}

/// Helper: log con un string traducido
#[macro_export]
macro_rules! log_t {
    (info, $field:ident) => {
        log::info!("{}", $crate::t!($field));
    };
    (info, $field:ident, $a:expr) => {
        log::info!("{}", $crate::t!($field).replacen("{}", &format!("{}", $a), 1));
    };
    (info, $field:ident, $a:expr, $b:expr, $c:expr) => {
        let s = $crate::t!($field).replacen("{}", &format!("{}", $a), 1);
        let s = s.replacen("{}", &format!("{}", $b), 1);
        log::info!("{}", s.replacen("{}", &format!("{}", $c), 1));
    };
    (warn, $field:ident) => {
        log::warn!("{}", $crate::t!($field));
    };
    (error, $field:ident) => {
        log::error!("{}", $crate::t!($field));
    };
    (error, $field:ident, $a:expr) => {
        log::error!("{}", $crate::t!($field).replacen("{}", &format!("{}", $a), 1));
    };
    (error, $field:ident, $a:expr, $b:expr, $c:expr) => {
        let s = $crate::t!($field).replacen("{}", &format!("{}", $a), 1);
        let s = s.replacen("{}", &format!("{}", $b), 1);
        log::error!("{}", s.replacen("{}", &format!("{}", $c), 1));
    };
    (debug, $field:ident) => {
        log::debug!("{}", $crate::t!($field));
    };
}

pub fn prompt_language() {
    use std::io::{self, Write};

    print!("{}", es::Es::LANGUAGE_PROMPT);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "1" | "spa" | "espanol" | "español" | "es" => {
            set_language(0);
            println!("{}", es::Es::LANGUAGE_SELECTED);
        }
        "2" | "eng" | "english" | "en" => {
            set_language(1);
            println!("{}", en::En::LANGUAGE_SELECTED);
        }
        _ => {
            set_language(0);
            println!("{}", es::Es::LANG_INVALID);
        }
    }
}
