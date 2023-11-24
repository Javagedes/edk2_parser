use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Result;
use configparser::ini::{Ini, IniDefault};
use indexmap::IndexMap as Map;
use log::{self, debug, trace, warn};
use regex::Regex;
use section::Edk2SectionEntry;

pub mod inf;
pub mod section;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub trait Config: Default {
    fn has_conditionals(&self) -> bool;
}
pub struct ConfigParser<T: Config> {
    config: T,
    pub lines: Option<Vec<String>>,
    map: Option<Map<String, Map<String, Option<String>>>>,
}
impl<T: Config> Default for ConfigParser<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Config> ConfigParser<T> {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            lines: None,
            map: None,
        }
    }

    pub fn get_section_entries<S: Edk2SectionEntry>(&self, scope: Option<&str>) -> Vec<S> {
        trace!(
            "get_section_entries: [section: {}] [scope: {}]",
            S::section_name(),
            scope.unwrap_or("None")
        );
        let section_name = S::section_name();
        let section_name = scope.map_or(section_name.to_string(), |s| {
            format!("{}.{}", section_name, s.to_lowercase())
        });

        let mut entries = Vec::new();
        let map = self.map.as_ref().expect("Nothing has been parsed.");
        let empty_map = Map::new();
        for search_pattern in self.create_search_patterns(section_name.clone()) {
            // Search in the current section/scope
            let found_entries = map
                .get(&search_pattern)
                .unwrap_or(&empty_map)
                .iter()
                .map(|(key, value)| S::new(key.clone(), value.clone()));
            for entry in found_entries.rev() {
                debug!(
                    "{} entry found in section [{}]",
                    S::section_name(),
                    search_pattern
                );
                trace!("Entry value: [{}]", entry);
                entries.insert(0, entry);
            }
        }

        entries
    }

    pub fn parse_from_file(&mut self, file_path: PathBuf) -> Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.parse(contents)
    }

    pub fn parse(&mut self, content: String) -> Result<()> {
        self.lines = Some(content.lines().map(|x| x.to_string()).collect());

        if self.config.has_conditionals() {
            self.process_conditionals()?;
        }

        self.expand_document()?;

        self.replace_macros()?;

        self.parse_ini()?;

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
        todo!()
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
        let mut macro_map: Map<String, Map<String, String>> = Map::new();
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
        macro_map: &Map<String, Map<String, String>>,
        current_section: String,
    ) -> Option<String> {
        println!("{}", current_section);
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

    fn parse_ini(&mut self) -> Result<()> {
        let mut defaults = IniDefault::default();
        defaults.comment_symbols = vec!['#'];
        defaults.case_sensitive = true;
        defaults.delimiters = vec!['='];

        let mut parser = Ini::new_from_defaults(defaults);

        let map = parser
            .read(
                self.lines
                    .as_ref()
                    .expect("No lines to parse.")
                    .join(LINE_ENDING),
            )
            .unwrap();

        self.map = Some(map);

        Ok(())
    }
}

#[cfg(test)]
mod config_parser_tests {
    use crate::{section::SourceEntry, Config, ConfigParser, LINE_ENDING};

    fn logger_init() {
        let _ = env_logger::builder().try_init();
    }

    #[derive(Default)]
    #[non_exhaustive]
    struct Cfg;
    impl Config for Cfg {
        fn has_conditionals(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_create_search_patterns() {
        logger_init();

        let parser = ConfigParser::<Cfg>::new();
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
        logger_init();

        let mut parser = ConfigParser::<Cfg>::new();
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
        logger_init();

        let mut parser = ConfigParser::<Cfg>::new();
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

    #[test]
    fn test_get_section_entries() {
        logger_init();

        let mut parser = ConfigParser::<Cfg>::new();
        let src = [
            "[Sources]",
            "  MyFile.c",
            "  MyFile2.c",
            "[Sources.IA32]",
            "  MyFile3.c",
            "  MyFile4.c",
            "[Sources.X64]",
            "  MyFile3.c",
            "  MyFile4.c",
        ];
        parser
            .parse(src.join(LINE_ENDING))
            .expect("Failed to split sections");

        let results = parser.get_section_entries::<SourceEntry>(None);
        assert_eq!(results.len(), 2);
        assert_eq!(results.get(0).unwrap().path, "MyFile.c");
        assert_eq!(results.get(1).unwrap().path, "MyFile2.c");

        let results = parser.get_section_entries::<SourceEntry>(Some("IA32"));
        assert_eq!(results.len(), 4);
        assert_eq!(results.get(2).unwrap().path, "MyFile3.c");
        assert_eq!(results.get(3).unwrap().path, "MyFile4.c");
    }
}
