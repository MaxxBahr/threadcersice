use serde::{Serialize, Deserialize};
use crate::{Addresses, Communicate, Receive, RespondMessage, SendMessage};

#[derive(Serialize, Deserialize, Clone)]
struct Unit{
    addr: Addresses,
    msg: String,
}

impl Unit{
    pub fn new(msg: String) -> Unit{
        Unit{
            addr: Addresses::Unit,
            msg
        }
    }
}

impl Communicate for Unit{
    fn establish_connection(&self) -> bool {
        todo!()
    }
}

impl SendMessage for Unit{
    fn send_message(&mut self, msg: String) -> bool {
        todo!()
    }

    fn get_peer_address(&mut self) -> String {
        todo!()
    }
}

impl Receive for Unit{
    fn receive_message(&mut self) -> RespondMessage {
        todo!()
    }
    
    type Output = RespondMessage;
}