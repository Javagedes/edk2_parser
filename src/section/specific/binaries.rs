use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

const CREATE_CMD: &str = "
CREATE TABLE IF NOT EXISTS binaries (
    path TEXT,
    scope1 TEXT,
    scope2 TEXT,
    filetype TEXT NOT NULL,
    binpath TEXT NOT NULL,
    target TEXT NOT NULL,
    family TEXT NOT NULL,
    tagname TEXT NOT NULL,
    FOREIGN KEY (path) REFERENCES files(path)
);
";

const INSERT_CMD: &str = 
"INSERT INTO binaries (path, scope1, scope2, filetype, binpath, target, family, tagname)
VALUES (?, ?, ?, ?, ?, ?, ?, ?);
";

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, _: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    conn.execute(CREATE_CMD, []).unwrap();
    let mut statement = conn.prepare(INSERT_CMD).unwrap();
    let path = path.to_str().unwrap();

    for (key, _) in section {
        let parts: Vec<&str> = key.split('|').collect();

        let (filetype, bin_path, target, family, tagname) = match parts.len() {
            2 => (parts[0], parts[1], "*", "*", "*"),
            3 => (parts[0], parts[1], parts[2], "*", "*"),
            4 => (parts[0], parts[1], parts[2], parts[3], "*"),
            5 => (parts[0], parts[1], parts[2], parts[3], parts[4]),
            _ => {
                return Err(Error::InvalidFormat(
                    "<filetype>|<path>[|<target>[|<family>[|<tagname>]]]".to_string(),
                    key
                )
                .into())
            }
        };
        statement.execute((path, &scope1, &scope2, filetype, bin_path, target, family, tagname)).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod test_binaries_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap! {
            String::from("RAW|MyFile/file.ext") => None,
            String::from("RAW|MyFile/file.ext|*|MSFT") => None,
            String::from("RAW|MyFile/file.ext|RELEASE|*|MyTag") => None,
            String::from("RAW|MyFile/file.ext|*|*|MyTag") => None,
        };

        let conn = call_parse_fn(lines, Box::new(parse)).unwrap();
        let mut query = conn.prepare("SELECT filetype, binpath, target, family, tagname FROM binaries;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, String>(1).unwrap(),
                row.get::<usize, String>(2).unwrap(),
                row.get::<usize, String>(3).unwrap(),
                row.get::<usize, String>(4).unwrap(),
            ))
        }).unwrap();

        let results = vec![
            ("RAW", "MyFile/file.ext", "*", "*", "*"),
            ("RAW", "MyFile/file.ext", "*", "MSFT", "*"),
            ("RAW", "MyFile/file.ext", "RELEASE", "*", "MyTag"),
            ("RAW", "MyFile/file.ext", "*", "*", "MyTag"),
        ];

        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.0, results[i].0);
            assert_eq!(row.1, results[i].1);
            assert_eq!(row.2, results[i].2);
            assert_eq!(row.3, results[i].3);
            assert_eq!(row.4, results[i].4);
        }
    }
    
}
