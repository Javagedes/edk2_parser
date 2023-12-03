use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::Result};

const CREATE_CMD: &str = "
CREATE TABLE IF NOT EXISTS depex (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    value TEXT NOT NULL,
    FOREIGN KEY (path) REFERENCES files(path)
    UNIQUE (scope1, scope2, value)
);
";

const INSERT_CMD: &str = 
"INSERT INTO depex (path, scope1, scope2, value) VALUES (?, ?, ?, ?);
";

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, _: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    conn.execute(CREATE_CMD, []).unwrap();
    let mut statement = conn.prepare(INSERT_CMD).unwrap();
    let path = path.to_str().unwrap();

    for (key, _) in section {
        let value = key.trim().to_string();

        statement.execute((path, &scope1, &scope2, value)).unwrap();
    }
    Ok(())
}


#[cfg(test)]
mod test_depex_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap! {
            String::from("TRUE1") => None,
            String::from("  TRUE2") => None,
            String::from("FALSE3") => None,
        };

        let conn = call_parse_fn(lines, Box::new(parse)).unwrap();
        let mut query = conn.prepare("SELECT * FROM depex;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((row.get::<usize, String>(0).unwrap(), row.get::<usize, String>(3).unwrap()))
        }).unwrap();

        let results = vec![
            "TRUE1".to_string(),
            "TRUE2".to_string(),
            "FALSE3".to_string(),
        ];

        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.1, results[i]);
        }
    }
    
}
