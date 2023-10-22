mod handler;
mod router;
mod server;
use server::Server;
fn main() {
    println!("Hello, world!");
    let server = Server::new("localhost:8080");
    server.run();
}
