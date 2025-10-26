use threadcersice::Client::Client;
use threadcersice::{Communicate, SendMessage};

fn main() {
    let mut client = Client::new("Hello".to_string());
    if client.establish_connection(){
        println!("Connection established successfully");
    };
    client.send_message("Hello from console".to_string());
    
}