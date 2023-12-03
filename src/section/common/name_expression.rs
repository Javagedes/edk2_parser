use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, section_name: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    
    let create_cmd = format!("
CREATE TABLE IF NOT EXISTS {} (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    name TEXT NOT NULL UNIQUE,
    expression TEXT,
    FOREIGN KEY (path) REFERENCES files(path)
);
", &section_name);

    let insert_cmd = format!("
INSERT INTO {} (path, scope1, scope2, name, expression)
VALUES (?, ?, ?, ?, ?);
", &section_name);
    conn.execute(&create_cmd, []).unwrap();

    let mut statement = conn.prepare(&insert_cmd).unwrap();
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
mod test_common_section_parser {
    use super::{*, super::test::call_parse_fn};

    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap::indexmap! {
            String::from("gEfiPeiMemoryDiscoveredPpiGuid") => None,
            String::from("gEfiPeiMemoryDiscoveredPpiGuid2|TRUE") => None,
        };

        let conn = call_parse_fn(lines, "commonsection", Box::from(parse)).unwrap();
        let mut query = conn.prepare("SELECT name, expression FROM commonsection;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((row.get::<usize, String>(0).unwrap(), row.get::<usize, Option<String>>(1).unwrap()))
        }).unwrap();

        let results = vec![
            ("gEfiPeiMemoryDiscoveredPpiGuid", None),
            ("gEfiPeiMemoryDiscoveredPpiGuid2", Some(String::from("TRUE")))];
        
        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.0, results[i].0);
            assert_eq!(row.1, results[i].1);
        }
    }
}
