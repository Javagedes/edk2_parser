#[cfg(test)]
mod tests {
    use edk2_parser::{inf::Inf, ConfigParser};
    use git2;
    use glob;
    use std;

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
        println!("pattern: {}", pattern);
        glob::glob(pattern.as_str()).unwrap().for_each(|entry| {
            let mut parser = ConfigParser::<Inf>::new();
            let path = entry.unwrap();
            parser.parse_from_file(path).unwrap();
        });
    }
}
