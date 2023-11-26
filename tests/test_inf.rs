#[cfg(test)]
mod tests {
    use edk2_parser::{
        inf::Inf,
        section::{
            BuildOptionEntry, FeaturePcdEntry, LibraryClassEntry, PackageEntry, PcdEntry,
            SourceEntry,
        },
        *,
    };
    use env_logger::{self, Target};

    fn logger_init() {
        let _ = env_logger::builder().target(Target::Stdout).try_init();
    }

    #[test]
    fn test_base_lib() {
        logger_init();

        let data = include_str!("data/baseLib.inf").to_string();
        let mut infp = ConfigParser::<Inf>::new();
        infp.parse(data).unwrap();

        assert_eq!(
            infp.get_section_entries::<SourceEntry>(None).unwrap().len(),
            37
        );
        assert_eq!(
            infp.get_section_entries::<SourceEntry>(Some("IA32"))
                .unwrap()
                .len(),
            180
        );
        assert_eq!(
            infp.get_section_entries::<SourceEntry>(Some("x64"))
                .unwrap()
                .len(),
            150
        );
        assert_eq!(
            infp.get_section_entries::<SourceEntry>(Some("IA32"))
                .unwrap()
                .iter()
                .filter(|s| s.family == Some("MSFT".to_string()))
                .count(),
            89
        );
        assert_eq!(
            infp.get_section_entries::<SourceEntry>(Some("IA32"))
                .unwrap()
                .iter()
                .filter(|s| s.family == Some("GCC".to_string()))
                .count(),
            23
        );

        assert_eq!(
            infp.get_section_entries::<LibraryClassEntry>(Some("ia32"))
                .unwrap()
                .len(),
            4
        );

        assert_eq!(infp.get_section_entries::<PcdEntry>(None).unwrap().len(), 5);
        assert_eq!(
            infp.get_section_entries::<FeaturePcdEntry>(None)
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn parse_openssl_lib() {
        logger_init();

        let data = include_str!("data/opensslLib.inf").to_string();
        let mut infp = ConfigParser::<Inf>::new();
        infp.parse(data).unwrap();

        assert!(!infp
            .get_section_entries::<SourceEntry>(None)
            .unwrap()
            .iter()
            .any(|s| s.path.contains("$(OPENSSL_PATH)")),);
        assert!(infp
            .get_section_entries::<SourceEntry>(None)
            .unwrap()
            .iter()
            .any(|s| s.path == "openssl/crypto/asn1/x_sig.c"),);

        assert_eq!(
            infp.get_section_entries::<LibraryClassEntry>(None)
                .unwrap()
                .len(),
            4
        );
        assert_eq!(
            infp.get_section_entries::<LibraryClassEntry>(Some("ARM"))
                .unwrap()
                .len(),
            5
        );
        assert!(infp
            .get_section_entries::<LibraryClassEntry>(Some("ARM"))
            .unwrap()
            .iter()
            .any(|s| s.name == "ArmSoftFloatLib"),);

        assert_eq!(
            infp.get_section_entries::<PackageEntry>(Some("common"))
                .unwrap()
                .len(),
            2
        );

        assert_eq!(
            infp.get_section_entries::<BuildOptionEntry>(None)
                .unwrap()
                .iter()
                .filter(|s| s.family == Some("MSFT".to_string()))
                .count(),
            6
        );
        assert_eq!(
            infp.get_section_entries::<BuildOptionEntry>(None)
                .unwrap()
                .iter()
                .filter(|s| s.family == Some("INTEL".to_string()))
                .count(),
            2
        );
        assert_eq!(
            infp.get_section_entries::<BuildOptionEntry>(None)
                .unwrap()
                .iter()
                .filter(|s| s.family == Some("GCC".to_string()))
                .count(),
            10
        );
        assert_eq!(
            infp.get_section_entries::<BuildOptionEntry>(None)
                .unwrap()
                .iter()
                .filter(|s| s.arch == *"IA32")
                .count(),
            5
        );
        assert_eq!(
            infp.get_section_entries::<BuildOptionEntry>(None)
                .unwrap()
                .iter()
                .filter(|s| s.value.contains("$(OPENSSL_FLAGS)"))
                .count(),
            0
        );
        assert_eq!(
            infp.get_section_entries::<BuildOptionEntry>(None)
                .unwrap()
                .iter()
                .filter(|s| s.value.contains("$(OPENSSL_FLAGS_CONFIG)"))
                .count(),
            0
        );
    }

    #[test]
    fn parse_reset_runtime_dxe() {
        logger_init();

        let data = include_str!("data/ResetRuntimeDxe.inf").to_string();
        let mut infp = ConfigParser::<Inf>::new();
        infp.parse(data).unwrap();

        assert_eq!(
            infp.get_section_entries::<SourceEntry>(None).unwrap().len(),
            1
        );
    }
}
