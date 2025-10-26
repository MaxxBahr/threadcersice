use threadcersice::Receive;
use threadcersice::Server::Server;
fn main() {
    let mut server = Server::new();
    server.receive_message();
}