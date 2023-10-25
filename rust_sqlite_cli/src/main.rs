use rusqlite::Connection;
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

        if let Some(validated_query) = validate_query_lifetime(&query) {
            match db_logic::execute_query(&conn, validated_query) {
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
        } else {
            println!("Invalid query format!");
        }
    }
}

fn validate_query_lifetime<'a>(query: &'a str) -> Option<&'a str> {
    // Dummy check: for this example, we assume any query that contains the word "SELECT" is valid.
    if query.contains("SELECT") || query.contains("INSERT") || query.contains("UPDATE") || query.contains("DELETE")||query.contains("PRAGMA") {
        Some(query)
    } else {
        None
    }
}
