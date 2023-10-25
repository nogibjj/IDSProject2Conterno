use rusqlite::{Connection, types::Value};
use rust_sqlite_cli::db_logic::execute_query;




#[test]
fn test_movies_table_structure() {
    let conn = Connection::open("movies.db").unwrap();
    
    let info = execute_query(&conn, "PRAGMA table_info(movies)").unwrap();
    
    // Check number of columns
    assert_eq!(info.len(), 13);
    
    // Check column names and types
    let expected = vec![
        ("id", "INTEGER"),
        ("imdb_id", "VARCHAR(20)"),
        ("title", "VARCHAR(255)"),
        ("director", "VARCHAR(255)"),
        ("year", "INT"),
        ("rating", "VARCHAR(8)"),
        ("genres", "VARCHAR(255)"),
        ("runtime", "INT"),
        ("country", "VARCHAR(255)"),
        ("language", "VARCHAR(255)"),
        ("imdb_score", "NUMERIC"),
        ("imdb_votes", "INT"),
        ("metacritic_score", "NUMERIC"),
    ];
    
    for (i, (name, data_type)) in expected.iter().enumerate() {
        let row = &info[i];
        assert_eq!(row[1], Value::Text(name.to_string()));
        assert_eq!(row[2], Value::Text(data_type.to_string()));
    }
}
