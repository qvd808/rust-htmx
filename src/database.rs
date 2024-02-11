use sqlite::{State, Connection};
use crate::item::Item;

struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Database {
        let connection = sqlite::open("database.db").unwrap();
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

    pub fn add_item(&self, item: Item) {
        self.connection
            .execute(
                format!("INSERT INTO items (name, description) VALUES ('{}', '{}')", item.get_name(), item.get_description()).as_str(),
            )
            .unwrap();
    }

    pub fn get_all_items(&self) -> Vec<Item> {
        let mut statement = self.connection
            .prepare("SELECT name, description FROM items")
            .unwrap();

        let mut items = Vec::new();
        while let State::Row = statement.next().unwrap() {
            items.push(Item::new(
                statement.read::<String, _>(0).unwrap(),
                statement.read::<String, _>(1).unwrap(),
            ));
        }
        items
    }
}