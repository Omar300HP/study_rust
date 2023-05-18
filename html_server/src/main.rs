fn main() {
    let server = Server::new(String::from("127.0.0.1"), String::from("8080"));

    server.run();

    println!(
        "Server is running on ip:{}, port:{}, address:{}",
        server.ip, server.port, server.address
    );
}

struct Server {
    ip: String,
    port: String,
    address: String,
}

impl Server {
    fn new(ip: String, port: String) -> Self {
        Self {
            address: format!("{}:{}", &ip, &port),
            ip,
            port,
        }
    }

    fn run(&self) {}
}
