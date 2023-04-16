use std::{
    env, fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
    thread,
    time::Duration,
};
use webserver::ThreadPool;

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

    let port = serialport::new(port, 9600)
        .open()
        .expect("Failed to open serial port");

    /*
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
    */
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let request_line = &http_request[0];
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
