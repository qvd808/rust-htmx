use sqlite::{State, Connection};
use crate::item::Item;
use sqlite::Error;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Database {
        let connection = Connection::open("database.db").unwrap();
        Database { connection }
    }

    pub fn create_table(&self) {
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS items (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL
                )",
            )
            .unwrap();
    }

    pub fn add_item(&self, item: Item) -> Result<(), Error>{
        self.connection
            .execute(
                format!("INSERT INTO items (name, description) VALUES ('{}', '{}')", item.get_name(), item.get_description()).as_str(),
            )
            .unwrap();
        Ok(())
    }

    pub fn get_all_items(&self) -> Result<Vec<Item>, Error> {

        let res = self.connection
            .prepare("SELECT id, name, description FROM items");

        match res {
            Ok(mut statement) => {
                let mut items = Vec::new();
                while let State::Row = statement.next().unwrap() {
                    items.push(Item::new(
                        
                        Some(statement.read::<i64, _>(0).unwrap()),
                        statement.read::<String, _>(1).unwrap(),
                        statement.read::<String, _>(2).unwrap(),
                    ));
                }
                Ok(items)
            }
            Err(e) => Err(e),
        }

       
    }

    pub fn get_item_with_id(&self, id: i64) -> Option<Item> {

        let mut statement = self.connection
            .prepare(format!("SELECT id, name, description FROM items WHERE id = {}", id).as_str())
            .unwrap();
        
        let item = statement.next().unwrap();
        match item {
            State::Row => Some(Item::new(
                Some(statement.read::<i64, _>(0).unwrap()),
                statement.read::<String, _>(1).unwrap(),
                statement.read::<String, _>(2).unwrap(),
            )),
            State::Done => None,
        }
        
    }

    pub fn update_item(&self, item: Item) -> Result<(), Error> {
        match item.id {
            None => Err(Error { code: Some(0), message: Some("Item does not exist".to_string()) } ),
            Some(_) => {
                self.connection
                    .execute(
                        format!("UPDATE items SET name = '{}', description = '{}' WHERE id = {}", item.get_name(), item.get_description(), item.id.unwrap()).as_str(),
                    )
                    .unwrap();
                Ok(())
            },
        }

    }
}