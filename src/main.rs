use hotkey_listener::{HotkeyEvent, HotkeyListenerBuilder, HotkeyListenerHandle};
use std::env;
use std::process::Command;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;
mod config;

fn keyeventcatcher(
    handle: HotkeyListenerHandle,
    tx: Sender<usize>,
    configs: Arc<Vec<config::HotkeyMapping>>,
) {
    loop {
        match handle.recv_timeout(Duration::from_millis(100)) {
            Ok(HotkeyEvent::Pressed(idx)) => {
                if &configs[idx].on_down == "1" {
                    tx.send(idx).expect("failed to send press");
                    println!("pressed {:?}", idx);
                }
            }
            Ok(HotkeyEvent::Released(idx)) => {
                if &configs[idx].on_down == "0" {
                    tx.send(idx).expect("failed to send release");
                    println!("released {:?}", idx)
                }
            }
            Err(_) => { /* timeout, check exit conditions */ }
        }
    }
}

fn keyeventhandeler(rx: Receiver<usize>, configs: Arc<Vec<config::HotkeyMapping>>) {
    loop {
        match rx.try_recv() {
            Ok(idx) => handlekey(&configs[idx]),
            Err(_) => { /* err handel */ }
        }
    }
}

fn handlekey(config: &config::HotkeyMapping) {
    let compcon = Command::new(config.condition.clone())
        .output()
        .expect("failed to execute compcon");

    let cond = String::from_utf8(compcon.stdout).unwrap();
    // println!("con: {}, con_val: {}", cond, &config.con_value);

    if cond.strip_suffix("\n").unwrap() == config.con_value {
        println!("executing: {} {}", config.cmd, config.args);

        let out = Command::new(config.cmd.clone())
            .arg(config.args.clone())
            .output()
            .expect("failed to execute");

        println!("output: {}", String::from_utf8(out.stdout).unwrap());
        return
    }
    println!("didnt execute, condition wasnt met");


}

fn main() {
    let (tx, rx) = channel();

    let args: Vec<String> = env::args().collect();

    let mut hotkey_builder = HotkeyListenerBuilder::new();
    let config_list = match config::parse_config(&args[1]) {
        Some(l) => l,
        None => panic!("couldnt load configfile"),
    };

    for config in &config_list {
        hotkey_builder = hotkey_builder.add_hotkey(config.hotkey.clone());
    }
    let sharded_config = Arc::new(config_list);
    // Build and start the listener - no manual shutdown flag needed
    let handle = hotkey_builder
        .build()
        .expect("failed to build")
        .start()
        .expect("failed to start");

    let config_list_send = Arc::clone(&sharded_config);
    let config_list_recv = Arc::clone(&sharded_config);

    let handle_send = thread::spawn(move || keyeventcatcher(handle, tx, config_list_send));

    let handle_recv = thread::spawn(move || keyeventhandeler(rx, config_list_recv));

    handle_send.join().unwrap();
    handle_recv.join().unwrap();
}
