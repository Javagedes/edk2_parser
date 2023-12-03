use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, section_name: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    let create_cmd = format!("
CREATE TABLE IF NOT EXISTS {} (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    tokenspace TEXT NOT NULL,
    name TEXT NOT NULL,
    value TEXT,
    expression TEXT,
    FOREIGN KEY (path) REFERENCES files(path),
    UNIQUE(tokenspace, name, path, scope1, scope2)
);", section_name);

    let insert_cmd = format!("
INSERT INTO {} (path, scope1, scope2, tokenspace, name, value, expression)
VALUES (?, ?, ?, ?, ?, ?, ?);
", section_name);
    
    conn.execute(&create_cmd, []).unwrap();

    let mut statement = conn.prepare(&insert_cmd).unwrap();
    let path = path.to_str().unwrap();
    let err = "<token_space>.<name>[|<value>][|<expression>]".to_string();

    for (key, _) in section{
        let parts: Vec<&str> = key.split('|').collect();

        let (token_space, name, value, expression) = match parts.len() {
            1 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(Error::InvalidFormat(err.clone(), key.clone()))?;
                (name, value, None, None)
            }
            2 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(Error::InvalidFormat(err.clone(), key.clone()))?;
                (name, value, Some(parts[1]), None)
            }
            3 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(Error::InvalidFormat(err.clone(), key.clone()))?;
                (name, value, Some(parts[1]), Some(parts[2]))
            }
            _ => return Err(Error::InvalidFormat(err, key).into()),
        };

        statement.execute((path, &scope1, &scope2, token_space, name, value, expression)).unwrap();
    }
    Ok(())
}


#[cfg(test)]
mod test_common_pcd_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap! {
            String::from("gTokenSpace.PcdMyFeature") => None,
            String::from("gTokenSpace.PcdMyFeature2|TRUE") => None,
            String::from("gTokenSpace.PcdMyFeature3|L\"HELLO\"|TRUE") => None,
        };

        let conn = call_parse_fn(lines, "featurepcds", Box::from(parse)).unwrap();
        let mut query = conn.prepare("SELECT tokenspace, name, value, expression FROM featurepcds;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, String>(1).unwrap(),
                row.get::<usize, Option<String>>(2).unwrap(),
                row.get::<usize, Option<String>>(3).unwrap(),
            ))
        }).unwrap();

        let results = vec![
            ("gTokenSpace", "PcdMyFeature", None, None),
            ("gTokenSpace", "PcdMyFeature2", Some("TRUE".to_string()), None),
            ("gTokenSpace", "PcdMyFeature3", Some("L\"HELLO\"".to_string()), Some("TRUE".to_string())),
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
