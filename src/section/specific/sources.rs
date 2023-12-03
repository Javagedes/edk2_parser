use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

const CREATE_CMD: &str = "
CREATE TABLE IF NOT EXISTS sources (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    src_path TEXT NOT NULL,
    family TEXT,
    FOREIGN KEY (path) REFERENCES files(path)
    UNIQUE (src_path, scope1, scope2)
);
";

const INSERT_CMD: &str = "
INSERT INTO sources (path, scope1, scope2, src_path, family) VALUES (?, ?, ?, ?, ?);
";

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, _: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    conn.execute(CREATE_CMD, []).unwrap();

    let mut statement = conn.prepare(INSERT_CMD).unwrap();
    let path = path.to_str().unwrap();

    for (key, _) in section {
        let parts: Vec<&str> = key.split('|').collect();

        let (src_path, family) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(parts[1])),
            _ => return Err(Error::InvalidFormat("<path>[|<family>]".to_string(), key).into()),
        };

        statement.execute((path, &scope1, &scope2, src_path, family)).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod test_sources_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines = indexmap! {
            String::from("MyFile.c") => None,
            String::from("MyFile2.c|MSFT") => None,
        };

        let conn = call_parse_fn(lines, Box::new(parse)).unwrap();
        let mut query = conn.prepare("SELECT src_path, family FROM sources;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, Option<String>>(1).unwrap()))
        }).unwrap();

        let results = vec![
            ("MyFile.c", None),
            ("MyFile2.c", Some(String::from("MSFT")))
        ];
        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.0, results[i].0);
            assert_eq!(row.1, results[i].1);
        }
    }
}