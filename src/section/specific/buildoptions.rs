use rusqlite::Connection;
use std::path::PathBuf;

use crate::{Edk2Section, error::{Error, Result}};

const CREATE_CMD: &str = "
CREATE TABLE IF NOT EXISTS buildoptions (
    path TEXT NOT NULL,
    scope1 TEXT,
    scope2 TEXT,
    family TEXT,
    target TEXT NOT NULL,
    tagname TEXT NOT NULL,
    arch TEXT NOT NULL,
    tool_code TEXT NOT NULL,
    attribute TEXT NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (path) REFERENCES files(path)
);
";

const INSERT_CMD: &str = 
"INSERT INTO buildoptions (path, scope1, scope2, family, target, tagname, arch, tool_code, attribute, value)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
";

pub fn parse(conn: &Connection, path: PathBuf, section: Edk2Section, _: String, scope1: Option<String>, scope2: Option<String>) -> Result<()> {
    conn.execute(CREATE_CMD, []).unwrap();
    let mut statement = conn.prepare(INSERT_CMD).unwrap();
    let path = path.to_str().unwrap();

    for (key, value) in section {
        let parts: Vec<&str> = key.split(&[':', '_'][..]).collect();

        let (family, target, tagname, arch, tool_code, attribute) = match parts.len() {
            5 => (None, parts[0], parts[1], parts[2], parts[3], parts[4]),
            6 => (
                Some(parts[0]),
                parts[1],
                parts[2],
                parts[3],
                parts[4],
                parts[5],
            ),
            _ => {
                return Err(Error::InvalidFormat(
                    "[<family>:]<target>_<tagname>_<arch>_<tool_code>_<attribute> = <value>".to_string(),
                    format!("{} = {:#?}", key, value)
                )
                .into())
            }
        };
        let family = family.map(|x| x.trim().to_string());
        let target = target.trim().to_string();
        let tagname = tagname.trim().to_string();
        let arch = arch.trim().to_string();
        let tool_code = tool_code.trim().to_string();
        let attribute = attribute.trim().to_string();
        let value = value.unwrap_or(String::from(""));

        statement.execute((path, &scope1, &scope2, family, target, tagname, arch, tool_code, attribute, value)).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod test_buildoptions_section_parser {
    use super::{*, super::test::call_parse_fn};
    use indexmap::indexmap;

    #[test]
    fn simple() {
        let lines: Edk2Section = indexmap! {
            String::from("RELEASE_VS2022_IA32_DLINK_FLAGS") => Some(String::from("")),
            String::from("MSFT:RELEASE_*_*_DLINK_PATH") => Some(String::from("C:\\link.exe")),
            String::from("MSFT:*_*_*_DL_FLAGS") => Some(String::from("MyValue")),
            String::from("MSFT:*_*_*_CC_FLAGS") => None,
        };
        let conn = call_parse_fn(lines, Box::new(parse)).unwrap();

        let mut query = conn.prepare("SELECT * FROM buildoptions;").unwrap();
        let rows = query.query_map([], |row| {
            Ok((
                row.get::<usize, Option<String>>(3).unwrap(),
                row.get::<usize, String>(4).unwrap(),
                row.get::<usize, String>(5).unwrap(),
                row.get::<usize, String>(6).unwrap(),
                row.get::<usize, String>(7).unwrap(),
                row.get::<usize, String>(8).unwrap(),
                row.get::<usize, String>(9).unwrap()
            ))
        }).unwrap();

        let results = vec![
            (None, "RELEASE", "VS2022", "IA32", "DLINK", "FLAGS", ""),
            (Some("MSFT".to_string()), "RELEASE", "*", "*", "DLINK", "PATH", "C:\\link.exe"),
            (Some("MSFT".to_string()), "*", "*", "*", "DL", "FLAGS", "MyValue"),
            (Some("MSFT".to_string()), "*", "*", "*", "CC", "FLAGS", ""),
        ];

        for (i, row) in rows.enumerate() {
            let row = row.unwrap();
            assert_eq!(row.0, results[i].0);
            assert_eq!(row.1, results[i].1);
            assert_eq!(row.2, results[i].2);
            assert_eq!(row.3, results[i].3);
            assert_eq!(row.4, results[i].4);
            assert_eq!(row.5, results[i].5);
            assert_eq!(row.6, results[i].6);
        }
    }
}