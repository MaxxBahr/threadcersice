use serde::{Serialize, Deserialize};

pub mod Client;
pub mod Server;
pub mod Unit;

#[derive(Serialize, Deserialize, Clone)]
enum Addresses{
    Client,
    Server,
    Unit
}

#[derive(PartialEq)]
struct RespondMessage{
    msg: String,
}

trait SendMessage: serde::Serialize + Clone
    {
        fn send_message(&self) -> bool;
        fn get_peer_address(&self) -> String;
}
    
trait Receive: for <'a> Deserialize<'a> + Clone
    {
        type Output: PartialEq;
        fn receive_message(&self) -> Output;
}

trait Communicate:  Receive<T, F> + Send + for<'a> Deserialize<'a> + Clone + Serialize
{
    fn establish_connection(&self) -> bool;
}
