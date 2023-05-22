use std::io::{Read, Write};
use std::net::TcpListener;
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
