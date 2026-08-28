use hotkey_listener::{Hotkey, parse_hotkey};
use configparser::ini::Ini;

use std::fs;

// #[derive(Deserialize, Debug)]
pub struct HotkeyMapping {
    pub idx: u8,
    pub cmd: String,
    pub args: Option<String>,
    pub hotkey: Hotkey
}


impl HotkeyMapping {
    pub fn new(idx: u8, cmd: &str, mut args: Option<&str>, hotkey: &str) -> Self{

        return HotkeyMapping{
            idx,
            cmd: String::from(cmd),
            args: if let Some(args) = args {
                Some(String::from(args))
            } else {
                None
            },
            hotkey: parse_hotkey(hotkey).unwrap()
        }
    }
}


pub fn parse_config(path: &str) {
    let content = fs::read_to_string(path).unwrap();

    let mut config = Ini::new();
    config.read(content).unwrap();

}