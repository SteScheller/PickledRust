use std::{
    env,
    process::exit,
    //thread,
    //time::Duration,
};

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

    /*
        TODO: this does not seem to work with virtual ports opened with socat

        thread 'main' panicked at 'Failed to open serial port: Error { kind: Unknown, description: "Not a typewriter" }'
    */

    let mut port = serialport::new(port, 9600)
        .open()
        .expect("Failed to open serial port");

    /*
    port.write(b"led off\r").unwrap();
    thread::sleep(Duration::from_secs(1));
    port.write(b"led red\r").unwrap();
    thread::sleep(Duration::from_secs(1));

    port.write(b"led off\r").unwrap();
    thread::sleep(Duration::from_secs(1));
    port.write(b"led green\r").unwrap();
    thread::sleep(Duration::from_secs(1));

    port.write(b"led off\r").unwrap();
    thread::sleep(Duration::from_secs(1));
    port.write(b"led blue\r").unwrap();
    thread::sleep(Duration::from_secs(1));

    port.write(b"led off\r").unwrap();
    */
    let app = Application::build("127.0.0.1:7878", port).expect("Failed to start application");
    app.run();

    println!("Shutting down.");
}
