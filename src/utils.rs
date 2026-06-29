use std::env::consts::OS;

pub fn get_dll_path(dll_path: &str) -> String {
    if dll_path.contains(".") {
        return dll_path.to_string();
    }
    
    format!("{dll_path}.{}", match OS {
        "windows" => "dll",
        "macos" => "dylib",
        _ => "so"
    })
}