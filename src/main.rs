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

fn keyeventhandeler(rx: Receiver<usize>, configs: Vec<config::HotkeyMapping>) {
    loop {
        match rx.try_recv() {
            Ok(idx) => handlekey(&configs[idx]),
            Err(_) => { /* err handel */}

        }
    }
}

fn handlekey(config: &config::HotkeyMapping) {

    let out = Command::new(config.cmd.clone())
        .arg(config.args.clone())
        .output()
        .expect("failed to execute");

    println!("{}", String::from_utf8(out.stdout).unwrap())



}

fn main() {
    let (tx, rx) = channel();

    let mut hotkey_builder = HotkeyListenerBuilder::new();
    let config_lists = match config::parse_config("/home/nils/Projects/RUST_PROJECTS/hotkeys2bash/config.ini"){
        Some(l) => l,
        None => panic!("couldnt load configfile"),
    };


    for config in &config_lists {
        hotkey_builder = hotkey_builder.add_hotkey(config.hotkey.clone());
    }


    // Build and start the listener - no manual shutdown flag needed
    let handle = hotkey_builder
        .build().expect("failed to build")
        .start().expect("failed to start");
    
    let handle_send = thread::spawn(move || {
        keyeventcatcher(handle, tx)
    });

    let handle_recv = thread::spawn(move || {
        keyeventhandeler(rx, config_lists)
    });

    handle_send.join().unwrap();
    handle_recv.join().unwrap();

}

