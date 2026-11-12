//! Windows registry read helpers for Image Line Sytrus.

pub const APP_KEY: &str = "Software\\Image\\Image Line Sytrus";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Image Line Sytrus";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Image Line Sytrus") },
        RegValue { name: "Version",     value: String::from("v2.7.24") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Image Line Sytrus") },
        RegValue { name: "Publisher",       value: String::from("Image") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Image Line Sytrus\\uninstall.exe\"") },
    ]
}
