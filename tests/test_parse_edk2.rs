#[cfg(test)]
mod tests {
    use edk2_parser::InfParser;

    #[test]
    fn parse_edk2_inf() {
        let repo_path = std::env::current_dir()
            .unwrap()
            .join("target")
            .join("tmp")
            .join("edk2");
        if !repo_path.exists() {
            git2::Repository::clone("https://github.com/tianocore/edk2.git", &repo_path)
                .unwrap()
                .workdir()
                .unwrap();
        }

        let pattern = format!("{}/**/*.{}", repo_path.display(), "inf");
        glob::glob(pattern.as_str()).unwrap().for_each(|entry| {
            let mut parser = InfParser::new(None);
            let path = entry.unwrap();
            parser.parse_file(path).unwrap();
        });
    }
}
