use postgres::{Connection, TlsMode};

use Load;
use Write as TodoWrite;

struct TodoItem {
    id: i32,
    item: String
}

pub struct PostgresDb;

impl Load for PostgresDb{
    fn load() -> Vec<String> {
        let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS todo (
                    id              SERIAL PRIMARY KEY,
                    item            VARCHAR NOT NULL
                  )", &[]).unwrap();
        
        let mut items: Vec<String> = Vec::new();

        for row in &conn.query("SELECT item FROM todo", &[]).unwrap() {
            items.push(row.get(0));
        };

        items
    }
}

impl TodoWrite for PostgresDb{
    fn write(items: Vec<String>) {
        let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();
        
        conn.execute("CREATE TABLE IF NOT EXISTS todo (
                    id              SERIAL PRIMARY KEY,
                    item            VARCHAR NOT NULL
                  )", &[]).unwrap();
        
        for item in items {
            conn.execute("INSERT INTO todo (item) VALUES ($1)",
                 &[&item]).unwrap();
        }
    }
}