use serde::{Serialize, Deserialize};

pub mod Client;
pub mod Server;
pub mod Unit;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Addresses{
    Client,
    Server,
    Unit
}

impl Addresses {
    pub fn to_bytes(&self) -> [u8; 4]{
        match self{
            Addresses::Client => 0_i32.to_be_bytes(),
            Addresses::Server => 1_i32.to_be_bytes(),
            Addresses::Unit => 2_i32.to_be_bytes(),
        }
    }
}

#[derive(PartialEq)]
pub struct RespondMessage{
    msg: String,
}

impl RespondMessage{
    pub fn new(msg: String) -> RespondMessage{
        RespondMessage{
            msg
        }
    }
}

pub trait SendMessage: serde::Serialize + Clone
    {
        fn send_message(&mut self, message: String) -> bool;
        fn get_peer_address(&mut self) -> String;
}
    
pub trait Receive: for <'a> Deserialize<'a> + Clone
    {
        type Output: PartialEq;
        fn receive_message(&mut self) -> Self::Output;
}

pub trait Communicate:  Receive + Send + for<'a> Deserialize<'a> + Clone + Serialize
{
    fn establish_connection(&self) -> bool;
}
