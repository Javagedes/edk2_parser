pub mod binaries;
pub mod defines;
pub mod depex;
pub mod guids;
pub mod libraryclasses;
pub mod sources;
pub mod buildoptions;
pub mod patchpcd;


#[cfg(test)]
mod test {
    use crate::{Edk2Section, ParseFn, error::Result};
    use rusqlite::Connection;

    pub fn call_parse_fn(section: Edk2Section, func: ParseFn) -> Result<Connection> {

        let conn = rusqlite::Connection::open_in_memory().unwrap();
        let path = std::path::PathBuf::from("test");
        let section_name = "".to_string();
        func(&conn, path, section, section_name, None, None).unwrap();
        Ok(conn)
    }
}
