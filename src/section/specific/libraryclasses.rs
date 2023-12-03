use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

const CREATE_CMD: &str = "
CREATE TABLE IF NOT EXISTS libraryclasses (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    name TEXT NOT NULL,
    expression TEXT,
    FOREIGN KEY (path) REFERENCES files(path)
    UNIQUE (name, scope1, scope2)
);
";

const INSERT_CMD: &str = "
INSERT INTO libraryclasses (path, scope1, scope2, name, expression) VALUES (?, ?, ?, ?, ?);
";

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, _: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    conn.execute(CREATE_CMD, []).unwrap();

    let mut statement = conn.prepare(INSERT_CMD).unwrap();
    let path = path.to_str().unwrap();

    for (key, _) in section {
        let parts: Vec<&str> = key.split('|').collect();

        let (name, expression) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(parts[1])),
            _ => return Err(Error::InvalidFormat("<name>[|<expression>]".to_string(), key).into()),
        };

        statement.execute((path, &scope1, &scope2, name, expression)).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod test_libraryclasses_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap! {
            String::from("HobLib") => None,
            String::from("HobLib2|TRUE") => None,
        };

        let conn = call_parse_fn(lines, Box::new(parse)).unwrap();
        let mut query = conn.prepare("SELECT * FROM libraryclasses;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((
                row.get::<usize, String>(3).unwrap(),
                row.get::<usize, Option<String>>(4).unwrap()
            ))
        }).unwrap();

        let results = vec![
            ("HobLib", None),
            ("HobLib2", Some(String::from("TRUE")))
        ];

        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.0, results[i].0);
            assert_eq!(row.1, results[i].1);
        }
    }
}