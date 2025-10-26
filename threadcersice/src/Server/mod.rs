use std::collections::HashMap;
use std::io::{BufRead, BufReader};
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
            let stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            for line in request{
                match line.as_str() {
                    "0" => self.store_addr(("Client".to_string(), Addresses::Client)),
                    "1" => self.store_addr(("Server".to_string(), Addresses::Server)),
                    "2" => self.store_addr(("Unit".to_string(), Addresses::Unit)),
                    _ => println!("Message: {}", line),
                }
            }
            println!("established connections with {:?}", self.addr_table);
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