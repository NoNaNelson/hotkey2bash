use hotkey_listener::{HotkeyEvent, Hotkey, HotkeyListenerBuilder, HotkeyListenerHandle, parse_hotkey};
use std::time::Duration;
use std::process::Command;
use std::thread;
use std::sync::mpsc::{Receiver, Sender, channel};
mod config;


fn keyeventcatcher(handle: HotkeyListenerHandle, tx: Sender<usize>) {
    loop {
        match handle.recv_timeout(Duration::from_millis(100)) {
            Ok(HotkeyEvent::Pressed(idx)) => {
                // TODO: figure out a way to map index of hotkey to defined hotkeys from builder
                tx.send(idx).expect("failed to send");
                println!("pressed {:?}", idx);

            },
            Ok(HotkeyEvent::Released(idx)) => {
                println!("released {:?}", idx)
            },
            Err(_) => { /* timeout, check exit conditions */ }
        }
    }
}

fn keyeventhandeler(rx: Receiver<usize>) {
    loop {
        match rx.try_recv() {
            Ok(idx) => handlekey(idx),
            Err(_) => { /* err handel */}

        }
    }
}

fn handlekey(idx: usize) {
    match idx {
        0 => {
            let out = Command::new("wlcrosshairctl").arg("toggle").output().expect("failed to execute");
            println!("{}", String::from_utf8(out.stdout).unwrap())
        },
        // add your commands here or whatever should happen
        // in order in which they were defined in main()
        // TODO: create config
        _ => println!("no matches"),
    }
}

fn main() {
    let (tx, rx) = channel();

    let mut hotkey_builder = HotkeyListenerBuilder::new();
    let c = config::parse_config("config.ini");



    hotkey_builder = hotkey_builder.add_hotkey(parse_hotkey("Shift+F10").unwrap())
    // add you hotkeys here
    ;


    // Build and start the listener - no manual shutdown flag needed
    let handle = hotkey_builder
        .build().expect("failed to build")
        .start().expect("failed to start");
    
    let handle_send = thread::spawn(move || {
        keyeventcatcher(handle, tx)
    });
    let handle_recv = thread::spawn(move || {
        keyeventhandeler(rx)
    });

    handle_send.join().unwrap();
    handle_recv.join().unwrap();

}

