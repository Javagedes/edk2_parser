use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

const CREATE_CMD: &str = "
CREATE TABLE IF NOT EXISTS patchpcd (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    tokenspace TEXT NOT NULL,
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    hex TEXT NOT NULL,
    FOREIGN KEY (path) REFERENCES files(path)
);
";

const INSERT_CMD: &str = "
INSERT INTO patchpcd (path, scope1, scope2, tokenspace, name, value, hex) VALUES (?, ?, ?, ?, ?, ?, ?);
";

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, _: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    conn.execute(CREATE_CMD, []).unwrap();

    let mut statement = conn.prepare(INSERT_CMD).unwrap();
    let path = path.to_str().unwrap();
    let err = "PatchPcdEntry must be in format <token_space>.<name>|<value>|<hex>".to_string();

    for (key, _) in section {
        let parts: Vec<&str> = key.split('|').collect();
        
        let (token_space, name, value, hex) = match parts.len() {
            3 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(Error::InvalidFormat(err.clone(), key.clone()))?;
                (name, value, parts[1], parts[2])
            }
            _ => return Err(Error::InvalidFormat(err, key).into()),
        };
        let token_space = token_space.trim().to_string();
        let name = name.trim().to_string();
        let value = value.trim().to_string();
        let hex = hex.trim().to_string();
        statement.execute((path, &scope1, &scope2, token_space, name, value, hex)).unwrap();
    }
    Ok(())
}


#[cfg(test)]
mod test_patchpcd_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines = indexmap! {
            String::from("gTokenSpace.PcdMyFeature|TRUE|0x1234") => None,
        };
        let conn = call_parse_fn(lines, Box::new(parse)).unwrap();
        let mut query = conn.prepare("SELECT * FROM patchpcd;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((
                row.get::<usize, String>(3).unwrap(),
                row.get::<usize, String>(4).unwrap(),
                row.get::<usize, String>(5).unwrap(),
                row.get::<usize, String>(6).unwrap(),
            ))
        }).unwrap();

        let results = vec![
            ("gTokenSpace", "PcdMyFeature", "TRUE", "0x1234"),
        ];

        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.0, results[i].0);
            assert_eq!(row.1, results[i].1);
            assert_eq!(row.2, results[i].2);
            assert_eq!(row.3, results[i].3);
        }
    }
    
}
