use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;
use serde::{Deserialize, Serialize};
use crate::{Addresses, Receive, SendMessage};

#[derive(Serialize, Deserialize, Clone)]
pub struct Server{
    addr: Addresses,
    addr_table: HashMap<String, Addresses>,
}

impl Server {
    pub fn new() -> Server {
        Server{
            addr: Addresses::Server,
            addr_table: HashMap::new()
        }
    }
}

impl Receive for Server{
    type Output = ();

    fn receive_message(&mut self) -> Self::Output {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buf_reader = BufReader::new(&stream);
            let message = &stream.read(&mut buf_reader).unwrap();
            let mut message = String::new();
            let request = buf_reader.read_to_string(&mut message);
        }
    }
}

impl Server{
    fn store_addr(&mut self, addr_pair: (String, Addresses)){
        self.addr_table.insert(addr_pair.0, addr_pair.1);
    }
}


impl SendMessage for Server{
    fn send_message(&mut self, msg: String) -> bool {
        todo!()
    }

    fn get_peer_address(&mut self) -> String {
        todo!()
    }
}