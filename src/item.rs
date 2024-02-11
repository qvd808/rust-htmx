
use serde::{Deserialize, Serialize};


#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Item {
    name: String,
    description: String,
}

impl Item {
    pub fn new(name: String, description: String) -> Item {
        Item {
            name,
            description,
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_description(&self) -> String{
        self.description.clone()
    }
}