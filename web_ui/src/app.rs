use std::{
    error::Error,
    fs,
    io::prelude::{Read, Write},
    net::{TcpListener, TcpStream},
};

use regex::Regex;
use serialport::SerialPort;

pub struct Application {
    listener: TcpListener,
    port: Box<dyn SerialPort>,
}

impl Application {
    pub fn build(addr: &str, port: Box<dyn SerialPort>) -> Result<Application, Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;

        Ok(Application { listener, port })
    }

    pub fn run(mut self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(stream, &mut self.port);
        }
    }

    fn handle_connection(mut stream: TcpStream, port: &mut Box<dyn SerialPort>) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Failed to read tcp stream");

        let request = String::from_utf8_lossy(&buffer[..]);
        let response = Self::execute_request(&request, port);

        stream.write_all(response.as_bytes()).unwrap();
    }

    fn execute_request(request: &str, port: &mut Box<dyn SerialPort>) -> String {
        let request_line = request.lines().collect::<Vec<_>>()[0];
        let (status_line, filename) = match &request_line[..] {
            "POST /set_led HTTP/1.1" => {
                let re_value = Regex::new(r"color=(\w+)").unwrap();
                if let Some(c) = re_value.captures(request){
                    let value = c.get(1).unwrap().as_str();
                    let command = format!("led {value}\r");
                    port.write(command.as_bytes())
                        .expect("Failed to send command via serial port");
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
        response
    }
}
