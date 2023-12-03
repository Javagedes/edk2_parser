use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

// Per the INF spec, defines do not allow scoping, however there are currently INFs in edk2 that
// do have scoping, so we need to support it. 
const CREATE_CMD: &str = "
CREATE TABLE IF NOT EXISTS defines (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (path) REFERENCES files(path),
    UNIQUE (scope1, scope2, key)
);
";

const INSERT_CMD: &str = 
"INSERT INTO defines (path, scope1, scope2, key, value) VALUES (?, ?, ?, ?, ?);
";

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, _: String, scope1: Option<String>, scope2: Option<String>)->Result<()> {
    conn.execute(CREATE_CMD, []).unwrap();
    let mut statement = conn.prepare(INSERT_CMD).unwrap();
    let path = path.to_str().unwrap();

    for (key, value) in section {
        
        let nkey = key.trim().to_uppercase();
        let nkey = nkey.trim_start_matches("DEFINE");
        match value {
            Some(value) => {
                let nkey = nkey.trim();
                let value = value.trim();
                statement.execute((path, &scope1, &scope2, nkey, value)).unwrap();
            },
            None => {
                return Err(Error::InvalidFormat("[DEFINE] <key> = <value>".to_string(), format!("{} = {:#?}", key, value)).into());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test_defines_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap! {
            String::from("DEFINE MYVAR") => Some("45".to_string()),
            String::from("MYVAR2") => Some("HELLO".to_string()),
        };
        let conn = call_parse_fn(lines, Box::new(parse)).unwrap();

        let mut query = conn.prepare("SELECT * FROM defines;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((
                row.get::<usize, String>(3).unwrap(),
                row.get::<usize, String>(4).unwrap(),
            ))
        }).unwrap();

        let results = vec![
            ("MYVAR", "45"),
            ("MYVAR2", "HELLO"),
        ];

        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.0, results[i].0);
            assert_eq!(row.1, results[i].1);
        }
    }
}
