use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePorts{
   pub ports_names:Vec<String>
}