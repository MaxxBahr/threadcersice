use serde::{Serialize, Deserialize};
use crate::{Addresses, Communicate, Receive, SendMessage, RespondMessage};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Clone)]
struct Client{
    addr: Addresses,
    msg: String,
}

impl Client {
    pub fn new(msg: String) -> Client {
        Client { addr: Addresses::Client, msg }
    }
}

impl Communicate for Client{
    fn establish_connection(&self) -> bool {
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        let msg_addr = self.addr.as_bytes();

        stream.write_all(msg_addr).unwrap();
        stream.flush().unwrap();
        true
    }
}

impl SendMessage for Client{
    fn send_message(&self) -> bool {
        todo!()
    }

    fn get_peer_address(&self) -> String {
        todo!()
    }
}

impl Receive for Client{
    fn receive_message(&self) -> RespondMessage {
        todo!()
    }
    
    type Output;
}