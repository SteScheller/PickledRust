use std::{
    error::Error,
    fs,
    io::prelude::{Read, Write},
    net::{TcpListener, TcpStream},
};

use serialport::SerialPort;

pub struct Application {
    listener: TcpListener,
    port: Box<dyn SerialPort>,
}

impl Application {
    pub fn build(addr: &str, port: Box<dyn SerialPort>) -> Result<Application, Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;

        Ok(Application { listener, port, })
    }

    pub fn run(self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
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
}
