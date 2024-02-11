
use serde::{Deserialize, Serialize};


#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Item {
    pub id: Option<i64>,
    name: String,
    description: String,
}

impl Item {
    pub fn new(id: Option<i64>, name: String, description: String) -> Item {
        match id {
            Some(id) => Item { id: Some(id), name, description },
            None => Item { id: None, name, description },
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_description(&self) -> String{
        self.description.clone()
    }
}