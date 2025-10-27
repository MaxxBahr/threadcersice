use std::{thread, time};
use threadcersice::Client::Client;
use threadcersice::{Communicate, SendMessage};

fn main() {
    let mut client = Client::new("Hello".to_string());
    if client.establish_connection(){
        println!("Connection established successfully");
    };
    let mut message_count: i32 = 0;
    let seconds = time::Duration::from_secs(3);
    loop{
    client.send_message(format!("Message nr. {message_count} sent!"));
        message_count += 1;
        thread::sleep(seconds);
    }

}