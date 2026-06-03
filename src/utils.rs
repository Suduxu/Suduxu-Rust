use std::env::consts::OS;

pub fn get_dll_path(dll_path: &str) -> String {
    format!("{dll_path}.{}", match OS {
        "windows" => "dll",
        "macos" => "dylib",
        _ => "so"
    })
}