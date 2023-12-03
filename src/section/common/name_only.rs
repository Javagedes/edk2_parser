use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::Result};

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, section_name: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    let create_cmd = format!("
CREATE TABLE IF NOT EXISTS {} (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    name TEXT NOT NULL,
    FOREIGN KEY (path) REFERENCES files(path)
);
", section_name);

    let insert_cmd = format!("
INSERT INTO {} (path, scope1, scope2, name) VALUES (?, ?, ?, ?);
", section_name);
    
    conn.execute(&create_cmd, []).unwrap();

    let mut statement = conn.prepare(&insert_cmd).unwrap();
    let path = path.to_str().unwrap();

    for (key, _) in section {
        let key = key.trim();
        
        statement.execute((path, &scope1, &scope2, key)).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod test_packages_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;
    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap! {
            String::from("MdePkg/MdePkg.dec") => None,
        };

        let conn = call_parse_fn(lines, "packages", Box::from(parse)).unwrap();
        let mut query = conn.prepare("SELECT name FROM packages;").unwrap();
        let mut rows = query.query_map([], |row| {
            Ok(row.get::<usize, String>(0).unwrap())
        }).unwrap();

        assert_eq!(rows.next().unwrap().unwrap(), "MdePkg/MdePkg.dec");
    }
}