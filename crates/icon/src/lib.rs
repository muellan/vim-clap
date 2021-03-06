mod constants;

use constants::{EXACTMATCH, EXACTMATCH_ICON, EXTENSION, EXTENSION_ICON};

use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

pub const DEFAULT_ICON: &str = "";
pub const DEFAULT_ICONIZED: &str = " ";
pub const FOLDER_ICON: &str = "";
pub const DEFAULT_FILER_ICON: &str = "";

/// The type used to represent icons.
///
/// This could be changed into different type later,
/// so functions take and return this type, not `char` or `&str` directly.
type Icon = &'static str;

/// Return appropriate icon for the path. If no icon matched, return the specified default one.
///
/// Try matching the exactmatch map against the file name, and then the extension map.
#[inline]
pub fn get_icon_or(path: &Path, default: Icon) -> Icon {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .and_then(
            |filename| match EXACTMATCH.binary_search(&filename.to_lowercase().as_str()) {
                Ok(idx) => Some(EXACTMATCH_ICON[idx]),
                Err(_) => None,
            },
        )
        .unwrap_or_else(|| {
            path.extension()
                .and_then(std::ffi::OsStr::to_str)
                .and_then(|ext| match EXTENSION.binary_search(&ext) {
                    Ok(idx) => Some(EXTENSION_ICON[idx]),
                    Err(_) => None,
                })
                .unwrap_or(&default)
        })
}

fn icon_for(line: &str) -> Icon {
    let path = Path::new(line);
    get_icon_or(&path, DEFAULT_ICON)
}

pub fn prepend_icon(line: &str) -> String {
    format!("{} {}", icon_for(line), line)
}

#[inline]
pub fn icon_for_filer(path: &Path) -> Icon {
    if path.is_dir() {
        FOLDER_ICON
    } else {
        get_icon_or(path, DEFAULT_FILER_ICON)
    }
}

pub fn prepend_filer_icon(path: &Path, line: &str) -> String {
    format!("{} {}", icon_for_filer(path), line)
}

pub fn prepend_grep_icon(line: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*):\d+:\d+:").unwrap();
    }
    let icon = RE
        .captures(line)
        .and_then(|cap| cap.get(1))
        .map(|m| icon_for(m.as_str()))
        .unwrap_or(DEFAULT_ICON);
    format!("{} {}", icon, line)
}
