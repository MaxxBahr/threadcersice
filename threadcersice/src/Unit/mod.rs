use serde::{Serialize, Deserialize};
use crate::{Addresses, Communicate, Receive, SendMessage};

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

impl Communicate<Unit, bool> for Unit{
    fn establish_connection(&self) -> bool {
        todo!()
    }
}

impl SendMessage<Unit> for Unit{
    fn send_message(&self) -> bool {
        todo!()
    }

    fn get_peer_address(&self) -> String {
        todo!()
    }
}

impl Receive for Unit{
    fn receive_message(&self) -> RespondMessage {
        todo!()
    }
    
    type Output;
}