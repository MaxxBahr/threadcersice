use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::{Communicate, Receive, SendMessage, Addresses, RespondMessage};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Clone)]
pub struct Client{
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
        let msg_adr = "0".to_string();
        let msg_addr = msg_adr.as_bytes();

        stream.write_all(&msg_addr).unwrap();
        stream.flush().unwrap();
        true
    }
}

impl SendMessage for Client{
    fn send_message(&mut self, msg: String) -> bool {
        let message = json!({
            "to": "Unit",
            "message": msg,
        });
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        if let Ok(()) =stream.write_all(message.to_string().as_bytes()){
            true
        }else{
            false
        }
    }

    fn get_peer_address(&mut self) -> String {
        todo!()
    }
}

impl Receive for Client{
    type Output = RespondMessage;
    
    fn receive_message(&mut self) -> crate::RespondMessage {
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        let mut buf = [0; 512];
        match stream.try_clone().ok().unwrap().read(&mut buf){
            Ok(size) if size > 0 => RespondMessage::new(format!("{:?}",&buf[..size])),
            _ => RespondMessage::new("No message delivered".to_string())
        }
    }
    
}