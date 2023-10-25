use rusqlite::{Connection, Result, types::Value};

pub fn execute_query(conn: &Connection, query: &str) -> Result<Vec<Vec<Value>>> {
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
