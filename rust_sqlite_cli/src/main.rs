use rusqlite::{Connection, Result, types::Value};

use std::io;

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

        match execute_query(&conn, query) {
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

fn execute_query(conn: &Connection, query: &str) -> Result<Vec<Vec<Value>>> {
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        let mut values = vec![];
        for i in 0..row.column_count() {
            values.push(row.get(i)?);
        }
        Ok(values)
    })?;

    let mut all_rows = vec![];
    for row_result in rows {
        match row_result {
            Ok(row) => all_rows.push(row),
            Err(e) => return Err(e),
        }
    }

    Ok(all_rows)
}
