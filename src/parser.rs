use std::{path::PathBuf, fs::File, io::Read, collections::HashMap};

use configparser::ini::Ini;
use indexmap::IndexMap;
use log::{debug, trace, warn};
use regex::Regex;
use rusqlite::Connection;

use crate::{config::Config, error::{Error, Result}};

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub(crate) type Edk2Section = IndexMap<String, Option<String>>;
pub(crate) type ParseFn = Box<dyn Fn(&Connection, PathBuf, Edk2Section, String, Option<String>, Option<String>)->Result<()>>;

pub struct ConfigParser<T: Config> {
    config: T,
    pub no_fail_mode: bool,
    lines: Option<Vec<String>>,
    connection: Connection,
    file: Option<PathBuf>,
}

impl<T: Config> Default for ConfigParser<T> {
    fn default() -> Self {
        Self::new(None)
    }
}
impl<T: Config> ConfigParser<T> {
    pub fn new(db_path: Option<PathBuf>) -> Self {
        let connection = match db_path {
            Some(path) => Connection::open(path).unwrap(),
            None => Connection::open_in_memory().unwrap(),
        };
        
        Self {
            config: T::default(),
            lines: None,
            connection,
            file: None,
            no_fail_mode: false,
        }
    }

    pub fn parse_file(&mut self, file_path: PathBuf) -> Result<()> {
        self.file = Some(file_path.clone());

        let mut contents = String::new();
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => return Err(Error::MissingFile(file_path.to_string_lossy().into())),
                    std::io::ErrorKind::PermissionDenied => return Err(Error::FileLocked(file_path.to_string_lossy().into())),
                    _ => return Err(Error::Unexpected),
                }
            }
        };
        
        if file.read_to_string(&mut contents).is_err() {
            return Err(Error::Unexpected)
        }

        self.parse(contents)
    }

    fn parse(&mut self, content: String) -> Result<()> {
        self.lines = Some(content.lines().map(|x| x.to_string()).collect());

        if self.config.has_conditionals() {
            self.process_conditionals()?;
        }

        self.expand_document()?;

        self.replace_macros()?;

        self.parse_config()?;

        Ok(())
    }

    fn parse_config(&mut self) -> Result<()> {
        let mut defaults = Ini::new_cs().defaults();
        defaults.comment_symbols = vec!['#'];
        defaults.delimiters = vec!['='];

        let mut parser = Ini::new_from_defaults(defaults);
        let section_parsers = self.config.supported_sections();

        let lines = self.lines.take().expect("No lines to parse.").join(LINE_ENDING);
        match parser.read(lines) {
            Ok(sections) => {
                for (section_name, section) in sections.iter() {
                    let parts: Vec<&str> = section_name.split('.').collect();
                    
                    let (section_name, scope1, scope2) = match parts.len() {
                        1 => (parts[0].to_lowercase(), None, None),
                        2 => (parts[0].to_lowercase(), Some(parts[1].to_lowercase()), None),
                        3 => (parts[0].to_lowercase(), Some(parts[1].to_lowercase()), Some(parts[2].to_lowercase())),
                        _ => {return Err(Error::InvalidFormat("<sectionname>[.<scope1>[.<scope2]]".to_string(), section_name.to_string()))}
                    };

                    let cmd = match section_parsers.get(section_name.as_str()) {
                        Some(cmd) => cmd,
                        None => {
                            return Err(Error::UnknownSection(section_name).into());
                        }
                    };
                    cmd(&self.connection, self.file.clone().unwrap(), section.clone(), section_name.clone(), scope1, scope2)?;
                }
            }
            Err(_) => {}
        }
        Ok(())
    }

    fn create_search_patterns(&self, search: String) -> Vec<String> {
        let mut pattern: Vec<String> = Vec::new();
        let mut sections: Vec<&str> = search.split('.').collect();
        if sections.len() == 3 {
            pattern.push(sections.join(".").to_string());
            if sections[1] != "common" {
                pattern.push(format!("{}.common.{}", sections[0], sections[2]));
            }
            sections.pop();
        }

        if sections.len() == 2 {
            if sections[1] != "common" {
                pattern.push(sections.join(".").to_string());
            }
            sections.pop();
        }

        if sections.len() == 1 {
            pattern.push(format!("{}.common", sections[0]));
            pattern.push(sections.join(".").to_string());
        }

        pattern.iter().map(|pat| pat.to_string()).collect()
    }

    fn process_conditionals(&mut self) -> Result<()> {
        //todo!()
        Ok(())
    }

    fn expand_document(&mut self) -> Result<()> {
        let mut output = Vec::new();
        let mut current_section = String::new();
        let mut section_contents: Vec<String> = Vec::new();

        let section_regex = Regex::new(r"^\s*\[(?P<section>[\w., ]+)\]\s*$").unwrap();

        for line in self.lines.take().expect("No lines to parse.") {
            if let Some(captures) = section_regex.captures(&line) {
                if !current_section.is_empty() {
                    // Write the contents of the previous section
                    self.write_document_sections(&mut output, &current_section, &section_contents);
                }

                // Start a new section
                current_section = captures["section"].to_lowercase();
                section_contents.clear();
            } else if !line.trim().is_empty() {
                // Add non-empty lines to the current section's contents
                section_contents.push(line.to_string());
            }
        }

        // Write the contents of the last section
        self.write_document_sections(&mut output, &current_section, &section_contents);

        self.lines = Some(output);
        Ok(())
    }

    fn write_document_sections(
        &self,
        output: &mut Vec<String>,
        sections: &str,
        contents: &[String],
    ) {
        for section in sections.split(',') {
            output.push(format!("[{}]", section.trim()));
            output.extend(contents.to_owned());
        }
    }

    fn replace_macros(&mut self) -> Result<()> {
        let mut output = Vec::new();
        let mut macro_map: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut current_section = String::new();

        let section_regex = Regex::new(r"^\s*\[(?P<section>[\w., ]+)\]\s*$").unwrap();
        let define_regex =
            Regex::new(r"(?i)^\s*DEFINE\s+(?P<variable>\w+)\s*=\s*(?P<value>.*)$").unwrap();
        let macro_regex = Regex::new(r"\$\((?P<macro>[^)]+)\)").unwrap();

        for line in self.lines.take().expect("No lines to parse.") {
            if line.trim().is_empty() || line.trim().starts_with('#') {
                output.push(line);
            } else if let Some(captures) = section_regex.captures(&line) {
                current_section = captures["section"].to_string();
                output.push(line);
            } else if let Some(captures) = define_regex.captures(&line) {
                let name = captures["variable"].to_string();
                let value = captures["value"].to_string();
                debug!("Macro found: {} = {}", name, value);
                macro_map
                    .entry(current_section.to_lowercase())
                    .or_default()
                    .insert(name, value);
            } else {
                let new_line = macro_regex
                    .replace_all(&line, |captures: &regex::Captures| {
                        let macro_name = captures["macro"].to_string();
                        match self.replace_macro(
                            macro_name.to_string(),
                            &macro_map,
                            current_section.to_lowercase(),
                        ) {
                            Some(replacement) => {
                                trace!("line: [{}]", line);
                                replacement.to_string()
                            }
                            None => {
                                warn!("Line: [{}]", line);
                                captures[0].to_string()
                            }
                        }
                    })
                    .to_string();
                output.push(new_line);
            }
        }

        self.lines = Some(output);
        Ok(())
    }

    fn replace_macro(
        &self,
        macro_name: String,
        macro_map: &HashMap<String, HashMap<String, String>>,
        current_section: String,
    ) -> Option<String> {
        let mut search_patterns = self.create_search_patterns(current_section.clone());
        search_patterns.push("defines".to_string());

        for search_pattern in &search_patterns {
            if let Some(value) = macro_map.get(search_pattern) {
                if let Some(value) = value.get(&macro_name) {
                    debug!(
                        "Macro Replaced: [{}]. Found in section [{}]",
                        macro_name, search_pattern
                    );
                    trace!("[{}] replaced with [{}]", macro_name, value);
                    return Some(value.to_string());
                }
            }
        }
        warn!(
            "Macro {} not found in any section. Searched sections: [{}]",
            macro_name,
            search_patterns.join(", ")
        );
        None
    }
}

#[cfg(test)]
mod config_parser_tests {
    use crate::InfParser;


    #[test]
    fn test_create_search_patterns() {
        let parser = InfParser::new(None);
        assert_eq!(
            parser.create_search_patterns("sources".to_string()),
            vec!["sources.common".to_string(), "sources".to_string()]
        );
        assert_eq!(
            parser.create_search_patterns("sources.ia32".to_string()),
            vec![
                "sources.ia32".to_string(),
                "sources.common".to_string(),
                "sources".to_string()
            ]
        );
        assert_eq!(
            parser.create_search_patterns("sources.common".to_string()),
            vec!["sources.common".to_string(), "sources".to_string()]
        );
        assert_eq!(
            parser.create_search_patterns("sources.ia32.dxe_driver".to_string()),
            vec![
                "sources.ia32.dxe_driver".to_string(),
                "sources.common.dxe_driver".to_string(),
                "sources.ia32".to_string(),
                "sources.common".to_string(),
                "sources".to_string()
            ]
        );
        assert_eq!(
            parser.create_search_patterns("sources.common.dxe_driver".to_string()),
            vec!["sources.common.dxe_driver", "sources.common", "sources"]
        );
    }
    #[test]
    fn test_expand_document() {
        let mut parser = InfParser::new(None);
        let src = [
            "[Sources]",
            "  Myfile.c",
            "  MyFile2.c",
            "[Sources.IA32, Sources.X64]",
            "  Myfile3.c",
            "  Myfile4.c",
        ];
        parser.lines = Some(src.iter().map(|x| x.to_string()).collect());

        parser.expand_document().expect("Failed to split sections");

        let result = vec![
            "[sources]",
            "  Myfile.c",
            "  MyFile2.c",
            "[sources.ia32]",
            "  Myfile3.c",
            "  Myfile4.c",
            "[sources.x64]",
            "  Myfile3.c",
            "  Myfile4.c",
        ];
        assert_eq!(parser.lines.unwrap(), result);
    }

    #[test]
    fn test_replace_macros() {
        let mut parser = InfParser::new(None);
        let src = vec![
            "[Defines]",
            "  DEFINE VAR = MyPath",
            "[Sources]",
            "  DEFINE  VAR1  =  MyPath1",
            "  $(VAR)/MyFile.c",
            "  $(VAR1)/Myfile1.c",
            "  MyFile2.c",
            "[Sources.IA32]",
            "  DEFINE VAR2 = MyPath2",
            "  $(VAR1)/Myfile3.c",
            "  $(VAR2)/Myfile4.c",
            "  $(VAR1)/$(VAR2)/MyFile5.c",
            "  $(VAR3)/Myfile6.c",
        ];
        parser.lines = Some(src.iter().map(|x| x.to_string()).collect());

        parser.replace_macros().expect("Failed to split sections");

        let result = vec![
            "[Defines]",
            "[Sources]",
            "  MyPath/MyFile.c",
            "  MyPath1/Myfile1.c",
            "  MyFile2.c",
            "[Sources.IA32]",
            "  MyPath1/Myfile3.c",
            "  MyPath2/Myfile4.c",
            "  MyPath1/MyPath2/MyFile5.c",
            "  $(VAR3)/Myfile6.c",
        ];
        assert_eq!(parser.lines.unwrap(), result);
    }
}
