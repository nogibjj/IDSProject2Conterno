use rusqlite::{Connection};
use std::io;

mod db_logic;

fn main() {
    let conn = Connection::open("movies.db").expect("Unable to open database");

    loop {
        // Prompt the user for input
        println!("Enter your SQL query (or 'exit' to quit):");
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("Failed to read line");

        let query = query.trim();

        if query == "exit" {
            break;
        }

        match db_logic::execute_query(&conn, query) {
            Ok(rows) => {
                for row in rows {
                    for (i, column) in row.iter().enumerate() {
                        if i != 0 {
                            print!(", ");
                        }
                        print!("{:?}", column);
                    }
                    println!();
                }
            }
            Err(e) => {
                println!("Error executing query: {}", e);
            }
        }
    }
}
