use std::string;

use serde::{Deserialize, Serialize};



#[derive(Debug, Clone,Serialize,Deserialize)]

pub struct NetWorkInfo {
    pub proto: String,
    pub local_port: u16,
    pub remote_addr:String,
    pub remote_port: u16,
    pub pid:String,
    pub state:String,
}