pub mod name_only;
pub mod pcds;
pub mod name_expression;

#[cfg(test)]
mod test {
    use crate::{ParseFn, Edk2Section, error::Result};
    use rusqlite::Connection;

    pub fn call_parse_fn(section: Edk2Section, section_name: &str, func: ParseFn) -> Result<Connection> {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        let path = std::path::PathBuf::from("test");
        func(&conn, path, section, section_name.to_string(), None, None).unwrap();
        Ok(conn)
    }
}
