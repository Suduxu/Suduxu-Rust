use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use ctrlc::set_handler;
use suduxu_rust::api::handler::suduxu::{addresses, config, for_client, init_suduxu, launch_suduxu, password, stop_suduxu, ON_CLIENT_CONNECTED, ON_CLIENT_DISCONNECTED};
use suduxu_rust::data::config::SuduxuBehaviourConfiguration;
use suduxu_rust::data::log::LogLevel;
use suduxu_rust::data::network::{VibrationStrength, VibrationType};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    init_suduxu(SuduxuBehaviourConfiguration::broadcast("DLL/suduxu")).expect("Failed to initialize Suduxu");

    set_handler(move || {
        r.store(false, Ordering::SeqCst);
        stop_suduxu().expect("Failed to stop Suduxu");
    })
        .unwrap();

    launch_suduxu().expect("Failed to launch Suduxu");

    ON_CLIENT_CONNECTED.subscribe(|id| {
        for_client(id).vibrate(750, VibrationStrength::Heavy, VibrationType::Custom);
        for_client(id).log(LogLevel::Error, "Client connected with error log", Some(format!("This is an error log for client {}", id).as_str()));
    });

    ON_CLIENT_DISCONNECTED.subscribe(|id| {
        println!("Client disconnected: {}", id);
    });

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(750));

        println!("Addresses: {:?}", addresses());
        println!("Config: {:?}", config());
        println!("Password: {:?}", password());
    });

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(200));
    }
}