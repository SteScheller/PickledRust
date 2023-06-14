use std::{
    env,
    fs,
    io::prelude::{Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
    //thread,
    //time::Duration,
};

mod threadpool;
use threadpool::ThreadPool;

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

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read tcp stream");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request:\n{}", request);

    let request_line = &request.lines().collect::<Vec<_>>()[0];
    let (status_line, filename) = match &request_line[..] {
        "POST /set_led HTTP/1.1" => {
            let key = "color=";
            if let Some(value_start) = request.find(key) {
                let value = &request[value_start + key.len()..];
                println!("{value}");
            }
            ("HTTP/1.1 200 OK", None)
        }
        _ => ("HTTP/1.1 200 OK", Some("index.html")),
    };

    let content = match filename {
        None => "".to_string(),
        Some(name) => match fs::read_to_string(name) {
            Ok(c) => c,
            Err(_) => "".to_string(),
        },
    };

    let length = content.len();
    let response = match length {
        0 => format!("{status_line}"),
        _ => format!("{status_line}\r\nContent-length: {length}\r\n\r\n{content}"),
    };
    stream.write_all(response.as_bytes()).unwrap();
}
