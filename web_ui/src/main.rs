use std::{env, process::exit};

use serialport;

mod app;
use crate::app::Application;

fn main() {
    let args: Vec<String> = env::args().collect();

    let port = match args.get(1) {
        Some(item) => item,
        None => {
            eprintln!("Missing required argument: serial port");
            exit(1);
        }
    };

    let addr = match args.get(2) {
        Some(item) => item,
        None => "127.0.0.1:7878",
    };

    let port = serialport::new(port, 9600)
        .open()
        .expect("Failed to open serial port");
    let app = Application::build(addr, port).expect("Failed to start application");
    app.run();

    println!("Shutting down.");
}
