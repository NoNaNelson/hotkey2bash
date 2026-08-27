use hotkey_listener::{parse_hotkey, HotkeyListenerBuilder, HotkeyEvent};
use std::time::Duration;
use std::process::Command;
use std::thread;


fn main() {
    let hotkey = parse_hotkey("Shift+F10").expect("failed to parse hotkey");
    let command = "wlcrosshairctl";
    let args = "toggle";


    // Build and start the listener - no manual shutdown flag needed
    let handle = HotkeyListenerBuilder::new()
        .add_hotkey(hotkey)
        .build().expect("failed to build")
        .start().expect("failed to start");
    loop {
        match handle.recv_timeout(Duration::from_millis(100)) {
            Ok(HotkeyEvent::Pressed(idx)) =>{
                // TODO: figure out a way to map index of hotkey to defined hotkeys from builder

                

                let output = Command::new(command)
                    .arg(args)
                    .output()
                    .expect("Failed to execute command");
                


                let output_str = String::from_utf8(
                output
                        .stdout
                        .as_slice()
                        .into()
                ).expect("failed to parse");

                println!("{:?}", output_str);



                },
            Ok(HotkeyEvent::Released(idx)) => println!("Hotkey {} released", idx),
            Err(_) => { /* timeout, check exit conditions */ }
        }
    }
}
