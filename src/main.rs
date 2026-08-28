use hotkey_listener::{HotkeyEvent, Hotkey, HotkeyListenerBuilder, HotkeyListenerHandle, parse_hotkey};
use std::time::Duration;
use std::process::Command;
use std::thread;
use std::sync::mpsc::{Receiver, Sender, channel};
mod config;


/*let receiver = thread::spawn(move || {
    let value = rx.recv().expect("Unable to receive from channel");
    println!("{value}");
});*/
fn keyeventcatcher(handle: HotkeyListenerHandle, tx: Sender<u8>) {
    loop {
        match handle.recv_timeout(Duration::from_millis(100)) {
            Ok(HotkeyEvent::Pressed(idx)) => {
                // TODO: figure out a way to map index of hotkey to defined hotkeys from builder
                // tx.send(idx);
                println!("pressed {:?}", idx);

            },
            Ok(HotkeyEvent::Released(idx)) => {
                println!("released {:?}", idx)
            },
            Err(_) => { /* timeout, check exit conditions */ }
        }
    }
}

fn keyeventhandeler(rx: Receiver<u8>) {
    match rx.try_recv() {
        Ok(idx) => println!("{}", idx),
        Err(_) => { /* err handel */}

    }

}
fn main() {
    let (tx, rx) = channel();

    let mut hotkey_builder = HotkeyListenerBuilder::new();
    let c = config::parse_config("config.ini");



    hotkey_builder = hotkey_builder.add_hotkey(parse_hotkey("F10").unwrap())

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

