use std::{fmt::Debug, path::StripPrefixError};
use hotkey_listener::{Hotkey, parse_hotkey};
use configparser::ini::Ini;

#[derive(Debug)]
pub struct HotkeyMapping {
    pub idx: u8,
    pub cmd: String,
    pub args: String,
    pub hotkey: Hotkey
}


pub fn parse_config(path: &str) -> Option<Vec<HotkeyMapping>>{

    let mut config = Ini::new();
    let mut mappings_list: Vec<HotkeyMapping>  = vec![];

    for (i, conf) in config.load(path).unwrap().into_iter().enumerate() {
        let parsed_args: Option<String> = match conf.1.get("args") {
            Some(val) => Some(val.clone().unwrap()),
            None => None
        };

        let x = HotkeyMapping {
            idx: i as u8,
            cmd: conf.1.get("cmd").unwrap().clone().unwrap(),
            hotkey: parse_hotkey(&conf.1.get("hotkey").unwrap().clone().unwrap()).unwrap(),
            args: conf
                .1
                .get("args")
                .unwrap()
                .clone()
                .unwrap_or_else(|| String::new()),
        };
        mappings_list.push(x);
    }
    if mappings_list.len() > 0 {
        return Some(mappings_list)
    } else {
        return None
    }
}