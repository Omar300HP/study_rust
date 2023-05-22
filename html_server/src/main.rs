mod http;
mod server;

use http::Method;
use http::Request;
use server::Server;
fn main() {
    let server = Server::new(String::from("127.0.0.1"), String::from("8080"));

    server.run();
}
