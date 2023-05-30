use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

use super::http::Request;
pub struct Server {
    ip: String,
    port: String,
    address: String,
}

impl Server {
    pub fn new(ip: String, port: String) -> Self {
        Self {
            address: format!("{}:{}", &ip, &port),
            ip,
            port,
        }
    }

    fn handle_connection(stream: &mut TcpStream) {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("src/hello.html".to_string()).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        println!(
            "Server is running on ip:{}, port:{}, address:{}",
            self.ip, self.port, self.address
        );

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 5120];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(
                                "Was able to read from connection {}",
                                String::from_utf8_lossy(&buffer)
                            );

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse request {}", e),
                            };

                            Server::handle_connection(&mut stream)
                        }
                        Err(error) => {
                            println!("Failed to read from connection {}", error);
                        }
                    };
                }
                Err(error) => {
                    println!("Failed to establish a connection {}", error);
                }
            };
        }
    }
}
