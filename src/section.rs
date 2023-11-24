use std::fmt::Debug;
use std::fmt::Display;

pub trait Edk2SectionEntry: Debug + Display {
    fn section_name() -> &'static str;

    fn new(key: String, value: Option<String>) -> Self;
}

/// Contains the parsed data from a single entry in the [Defines] section of an INF file.
///
/// # Define line format
///
/// As many define entries are structured differently, the value will need to be parsed further
/// by the caller.
/// ```text
///   <name> = <value>
/// ```
#[derive(Debug)]
pub struct DefineEntry {
    pub name: String,
    pub value: String,
}
impl Display for DefineEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}
impl Edk2SectionEntry for DefineEntry {
    fn section_name() -> &'static str {
        "defines"
    }
    fn new(key: String, value: Option<String>) -> Self {
        Self {
            name: key.trim().to_string(),
            value: value
                .expect("Define must be in format key = value")
                .trim()
                .to_string(),
        }
    }
}

/// Contains the parsed data from a single entry in the [Sources] section of an INF file.
///
/// # Define line format
///
/// ```text
///   path
/// ```
#[derive(Debug)]
pub struct SourceEntry {
    pub path: String,
    pub family: Option<String>,
}
impl Display for SourceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)?;

        if let Some(family) = &self.family {
            write!(f, "|{}", family)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for SourceEntry {
    fn section_name() -> &'static str {
        "sources"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "Source must be in format <path> [|<family>]";
        Self {
            path: parts.next().expect(err).trim().to_string(),
            family: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct BinaryEntry {
    pub filetype: String,
    pub path: String,
    pub target: String,
    pub tagname: String,
    pub family: String,
}
impl Display for BinaryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}",
            self.filetype, self.path, self.target, self.family, self.tagname
        )
    }
}
impl Edk2SectionEntry for BinaryEntry {
    fn section_name() -> &'static str {
        "binaries"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "Binary must be in format filetype|path";
        Self {
            filetype: parts.next().expect(err).trim().to_string(),
            path: parts.next().expect(err).trim().to_string(),
            target: parts.next().unwrap_or("*").trim().to_string(),
            family: parts.next().unwrap_or("*").trim().to_string(),
            tagname: parts.next().unwrap_or("*").trim().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ProtocolEntry {
    pub name: String,
    pub expression: Option<String>,
}
impl Display for ProtocolEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(expression) = &self.expression {
            write!(f, " | {}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for ProtocolEntry {
    fn section_name() -> &'static str {
        "protocols"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "Protocol must be in format <name> [|<expression>]";
        Self {
            name: parts.next().expect(err).trim().to_string(),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct PpiEntry {
    pub name: String,
    pub expression: Option<String>,
}
impl Display for PpiEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(expression) = &self.expression {
            write!(f, "|{}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for PpiEntry {
    fn section_name() -> &'static str {
        "ppis"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "PPI must be in format <name> [|<expression>]";
        Self {
            name: parts.next().expect(err).trim().to_string(),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct GuidEntry {
    pub name: String,
    pub expression: Option<String>,
}
impl Display for GuidEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(expression) = &self.expression {
            write!(f, "|{}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for GuidEntry {
    fn section_name() -> &'static str {
        "guids"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "GUID must be in format <name> [|<expression>]";
        Self {
            name: parts.next().expect(err).trim().to_string(),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct LibraryClassEntry {
    pub name: String,
    pub expression: Option<String>,
}
impl Display for LibraryClassEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(expression) = &self.expression {
            write!(f, "|{}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for LibraryClassEntry {
    fn section_name() -> &'static str {
        "libraryclasses"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "LibraryClass must be in format <name> [|<expression>]";
        Self {
            name: parts.next().expect(err).trim().to_string(),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct PackageEntry {
    pub name: String,
}
impl Display for PackageEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
impl Edk2SectionEntry for PackageEntry {
    fn section_name() -> &'static str {
        "packages"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        Self {
            name: key.trim().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FeaturePcdEntry {
    pub token_space: String,
    pub name: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}
impl Display for FeaturePcdEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.token_space, self.name)?;

        if let Some(value) = &self.value {
            write!(f, "|{}", value)?;
        }

        if let Some(expression) = &self.expression {
            write!(f, "|{}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for FeaturePcdEntry {
    fn section_name() -> &'static str {
        "featurepcd"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "FeaturePcd must be in format <token_space>.<name>[|<value>][|<expression>]";
        let (name, value) = parts.next().expect(err).split_once('.').expect(err);

        Self {
            token_space: name.trim().to_string(),
            name: value.trim().to_string(),
            value: parts.next().map(|x| x.trim().to_string()),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct FixedPcdEntry {
    pub token_space: String,
    pub name: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}
impl Display for FixedPcdEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.token_space, self.name)?;

        if let Some(value) = &self.value {
            write!(f, "|{}", value)?;
        }

        if let Some(expression) = &self.expression {
            write!(f, "|{}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for FixedPcdEntry {
    fn section_name() -> &'static str {
        "fixedpcd"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "FixedPcd must be in format <token_space>.<name>[|<value>][|<expression>]";
        let (name, value) = parts.next().expect(err).split_once('.').expect(err);

        Self {
            token_space: name.trim().to_string(),
            name: value.trim().to_string(),
            value: parts.next().map(|x| x.trim().to_string()),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}
#[derive(Debug)]
pub struct PatchPcdEntry {
    pub token_space: String,
    pub name: String,
    pub value: String,
    pub hex: String,
}
impl Display for PatchPcdEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}|{}|{}",
            self.token_space, self.name, self.value, self.hex
        )
    }
}
impl Edk2SectionEntry for PatchPcdEntry {
    fn section_name() -> &'static str {
        "patchpcd"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "PatchPcdEntry must be in format <token_space>.<name>|<value>|<hex>";
        let (name, value) = parts.next().expect(err).split_once('.').expect(err);

        Self {
            token_space: name.trim().to_string(),
            name: value.trim().to_string(),
            value: parts.next().expect(err).trim().to_string(),
            hex: parts.next().expect(err).trim().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct PcdEntry {
    pub token_space: String,
    pub name: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}
impl Display for PcdEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.token_space, self.name)?;

        if let Some(value) = &self.value {
            write!(f, "|{}", value)?;
        }

        if let Some(expression) = &self.expression {
            write!(f, "|{}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for PcdEntry {
    fn section_name() -> &'static str {
        "pcd"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "Pcd must be in format <token_space>.<name>[|<value>][|<expression>]";
        let (name, value) = parts.next().expect(err).split_once('.').expect(err);

        Self {
            token_space: name.trim().to_string(),
            name: value.trim().to_string(),
            value: parts.next().map(|x| x.trim().to_string()),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct PcdEx {
    pub token_space: String,
    pub name: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}
impl Display for PcdEx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.token_space, self.name)?;

        if let Some(value) = &self.value {
            write!(f, "|{}", value)?;
        }

        if let Some(expression) = &self.expression {
            write!(f, "|{}", expression)?;
        }

        Ok(())
    }
}
impl Edk2SectionEntry for PcdEx {
    fn section_name() -> &'static str {
        "pcdex"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        let mut parts = key.split('|');
        let err = "PcdEx must be in format <token_space>.<name>[|<value>][|<expression>]";
        let (name, value) = parts.next().expect(err).split_once('.').expect(err);

        Self {
            token_space: name.trim().to_string(),
            name: value.trim().to_string(),
            value: parts.next().map(|x| x.trim().to_string()),
            expression: parts.next().map(|x| x.trim().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct DepexEntry {
    value: String,
}
impl Display for DepexEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}
impl Edk2SectionEntry for DepexEntry {
    fn section_name() -> &'static str {
        "depex"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        Self {
            value: key.trim().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct UserExtensionEntry {
    value: String,
}
impl Display for UserExtensionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}
impl Edk2SectionEntry for UserExtensionEntry {
    fn section_name() -> &'static str {
        "userextensions"
    }
    fn new(key: String, _value: Option<String>) -> Self {
        Self {
            value: key.trim().to_string(),
        }
    }
}

/// [<family>:]<target>_<tagname>_<arch>_<tool_code>_<attribute> = <value>
/// if value starts with a "=", replace the entry if it exists.
/// otherwise, just append.
/// If a $() is in quotes, don't replace TODO: replace_macro probably breaks this
#[derive(Debug)]
pub struct BuildOptionEntry {
    pub family: Option<String>,
    pub target: String,
    pub tagname: String,
    pub arch: String,
    pub tool_code: String,
    pub attribute: String,
    pub value: String,
}
impl Display for BuildOptionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(family) = &self.family {
            write!(f, "{}:", family)?;
        }
        write!(
            f,
            "{}_{}_{}_{}_{} = {}",
            self.target, self.tagname, self.arch, self.tool_code, self.attribute, self.value
        )
    }
}
impl Edk2SectionEntry for BuildOptionEntry {
    fn section_name() -> &'static str {
        "buildoptions"
    }
    fn new(key: String, value: Option<String>) -> Self {
        let parts: Vec<&str> = key.split(&[':', '_'][..]).collect();

        let err = "BuildOption must be in format [<family>:]<target>_<tagname>_<arch>_<tool_code>_<attribute> = <value>";
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
            count => panic!("Only {} parts detected. {}", count, err),
        };

        Self {
            family: family.map(|x| x.trim().to_string()),
            target: target.trim().to_string(),
            tagname: tagname.trim().to_string(),
            arch: arch.trim().to_string(),
            tool_code: tool_code.trim().to_string(),
            attribute: attribute.trim().to_string(),
            value: value.unwrap_or("".to_string()),
        }
    }
}

#[cfg(test)]
mod section_entry_tests {
    use super::*;
    use env_logger;

    fn logger_init() {
        let _ = env_logger::builder().try_init();
    }

    #[test]
    fn test_define_entry() {
        logger_init();

        assert_eq!(DefineEntry::section_name(), "defines");
        let entry = DefineEntry::new("INF_VERSION".to_string(), Some(" 0x00010005".to_string()));
        assert_eq!(entry.name, "INF_VERSION");
        assert_eq!(entry.value, "0x00010005");
        assert_eq!(format!("{}", entry), "INF_VERSION = 0x00010005");
    }

    #[test]
    fn test_source_entry() {
        logger_init();

        assert_eq!(SourceEntry::section_name(), "sources");
        let entry = SourceEntry::new("  MyFile.c".to_string(), None);
        assert_eq!(entry.path, "MyFile.c");
        assert_eq!(format!("{}", entry), "MyFile.c");

        let entry = SourceEntry::new("MyFile.c | MSFT".to_string(), None);
        assert_eq!(entry.path, "MyFile.c");
        assert_eq!(entry.family, Some("MSFT".to_string()));
        assert_eq!(format!("{}", entry), "MyFile.c|MSFT");
    }

    #[test]
    fn test_binary_entry() {
        logger_init();

        assert_eq!(BinaryEntry::section_name(), "binaries");
        let entry = BinaryEntry::new("RAW|MyFile/file.ext".to_string(), None);
        assert_eq!(entry.filetype, "RAW");
        assert_eq!(entry.path, "MyFile/file.ext");
        assert_eq!(entry.target, "*");
        assert_eq!(entry.family, "*");
        assert_eq!(entry.tagname, "*");
        assert_eq!(format!("{}", entry), "RAW|MyFile/file.ext|*|*|*");

        let entry = BinaryEntry::new("RAW|MyFile/file.ext|*|MSFT".to_string(), None);
        assert_eq!(entry.filetype, "RAW");
        assert_eq!(entry.path, "MyFile/file.ext");
        assert_eq!(entry.target, "*");
        assert_eq!(entry.family, "MSFT");
        assert_eq!(entry.tagname, "*");
        assert_eq!(format!("{}", entry), "RAW|MyFile/file.ext|*|MSFT|*");

        let entry = BinaryEntry::new("RAW|MyFile/file.ext|RELEASE|*|MyTag".to_string(), None);
        assert_eq!(entry.filetype, "RAW");
        assert_eq!(entry.path, "MyFile/file.ext");
        assert_eq!(entry.target, "RELEASE");
        assert_eq!(entry.family, "*");
        assert_eq!(entry.tagname, "MyTag");
        assert_eq!(format!("{}", entry), "RAW|MyFile/file.ext|RELEASE|*|MyTag");

        let entry = BinaryEntry::new("RAW|MyFile/file.ext|*|*|MyTag".to_string(), None);
        assert_eq!(entry.filetype, "RAW");
        assert_eq!(entry.path, "MyFile/file.ext");
        assert_eq!(entry.target, "*");
        assert_eq!(entry.family, "*");
        assert_eq!(entry.tagname, "MyTag");
        assert_eq!(format!("{}", entry), "RAW|MyFile/file.ext|*|*|MyTag");
    }

    #[test]
    fn test_protocol_entry() {
        logger_init();

        assert_eq!(ProtocolEntry::section_name(), "protocols");
        let entry = ProtocolEntry::new("gEfiPciIoProtocolGuid".to_string(), None);
        assert_eq!(entry.name, "gEfiPciIoProtocolGuid");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gEfiPciIoProtocolGuid");

        let entry = ProtocolEntry::new("gEfiPciIoProtocolGuid|TRUE".to_string(), None);
        assert_eq!(entry.name, "gEfiPciIoProtocolGuid");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gEfiPciIoProtocolGuid | TRUE");
    }

    #[test]
    fn test_ppi_entry() {
        logger_init();

        assert_eq!(PpiEntry::section_name(), "ppis");
        let entry = PpiEntry::new("gEfiPeiMemoryDiscoveredPpiGuid".to_string(), None);
        assert_eq!(entry.name, "gEfiPeiMemoryDiscoveredPpiGuid");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gEfiPeiMemoryDiscoveredPpiGuid");

        let entry = PpiEntry::new("gEfiPeiMemoryDiscoveredPpiGuid|TRUE".to_string(), None);
        assert_eq!(entry.name, "gEfiPeiMemoryDiscoveredPpiGuid");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gEfiPeiMemoryDiscoveredPpiGuid|TRUE");
    }

    #[test]
    fn test_guid_entry() {
        logger_init();

        assert_eq!(GuidEntry::section_name(), "guids");
        let entry = GuidEntry::new("gEfiHobMemoryAllocModuleGuid".to_string(), None);
        assert_eq!(entry.name, "gEfiHobMemoryAllocModuleGuid");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gEfiHobMemoryAllocModuleGuid");

        let entry = GuidEntry::new("gEfiHobMemoryAllocModuleGuid|TRUE".to_string(), None);
        assert_eq!(entry.name, "gEfiHobMemoryAllocModuleGuid");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gEfiHobMemoryAllocModuleGuid|TRUE");
    }

    #[test]
    fn test_library_class_entry() {
        logger_init();

        assert_eq!(LibraryClassEntry::section_name(), "libraryclasses");
        let entry = LibraryClassEntry::new("MyLib".to_string(), None);
        assert_eq!(entry.name, "MyLib");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "MyLib");

        let entry = LibraryClassEntry::new("MyLib|TRUE".to_string(), None);
        assert_eq!(entry.name, "MyLib");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "MyLib|TRUE");
    }

    #[test]
    fn test_package_entry() {
        logger_init();

        assert_eq!(PackageEntry::section_name(), "packages");
        let entry = PackageEntry::new("MdePkg/MdePkg.dec".to_string(), None);
        assert_eq!(entry.name, "MdePkg/MdePkg.dec");
        assert_eq!(format!("{}", entry), "MdePkg/MdePkg.dec");
    }

    #[test]
    fn test_feature_pcd_entry() {
        logger_init();

        assert_eq!(FeaturePcdEntry::section_name(), "featurepcd");
        let entry = FeaturePcdEntry::new("gTokenSpace.PcdMyFeature|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature|TRUE");

        let entry = FeaturePcdEntry::new("gTokenSpace.PcdMyFeature".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature");

        let entry =
            FeaturePcdEntry::new("gTokenSpace.PcdMyFeature|L\"HELLO\"|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, Some("L\"HELLO\"".to_string()));
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(
            format!("{}", entry),
            "gTokenSpace.PcdMyFeature|L\"HELLO\"|TRUE"
        );
    }

    #[test]
    fn test_fixed_pcd_entry() {
        logger_init();

        assert_eq!(FixedPcdEntry::section_name(), "fixedpcd");
        let entry = FixedPcdEntry::new("gTokenSpace.PcdMyFeature|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature|TRUE");

        let entry = FixedPcdEntry::new("gTokenSpace.PcdMyFeature".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature");

        let entry =
            FixedPcdEntry::new("gTokenSpace.PcdMyFeature|L\"HELLO\"|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, Some("L\"HELLO\"".to_string()));
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(
            format!("{}", entry),
            "gTokenSpace.PcdMyFeature|L\"HELLO\"|TRUE"
        );
    }

    #[test]
    fn test_patch_pcd_entry() {
        logger_init();

        assert_eq!(PatchPcdEntry::section_name(), "patchpcd");
        let entry = PatchPcdEntry::new("gTokenSpace.PcdMyFeature|TRUE|0x1234".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, "TRUE");
        assert_eq!(entry.hex, "0x1234");
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature|TRUE|0x1234");
    }

    #[test]
    fn test_pcd_entry() {
        logger_init();

        assert_eq!(PcdEntry::section_name(), "pcd");
        let entry = PcdEntry::new("gTokenSpace.MyPcd|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd|TRUE");

        let entry = PcdEntry::new("gTokenSpace.MyPcd".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd");

        let entry = PcdEntry::new("gTokenSpace.MyPcd|L\"HELLO\"|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, Some("L\"HELLO\"".to_string()));
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd|L\"HELLO\"|TRUE");
    }

    #[test]
    fn test_pcd_ex_entry() {
        logger_init();

        assert_eq!(PcdEx::section_name(), "pcdex");
        let entry = PcdEx::new("gTokenSpace.MyPcd|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd|TRUE");

        let entry = PcdEx::new("gTokenSpace.MyPcd".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd");

        let entry = PcdEx::new("gTokenSpace.MyPcd|L\"HELLO\"|TRUE".to_string(), None);
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, Some("L\"HELLO\"".to_string()));
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd|L\"HELLO\"|TRUE");
    }

    #[test]
    fn test_depex_entry() {
        logger_init();

        assert_eq!(DepexEntry::section_name(), "depex");
        let entry = DepexEntry::new("TRUE".to_string(), None);
        assert_eq!(entry.value, "TRUE");
        assert_eq!(format!("{}", entry), "TRUE");
    }

    #[test]
    fn test_user_extension_entry() {
        logger_init();

        assert_eq!(UserExtensionEntry::section_name(), "userextensions");
        let entry = UserExtensionEntry::new("MyUserExtension".to_string(), None);
        assert_eq!(entry.value, "MyUserExtension");
        assert_eq!(format!("{}", entry), "MyUserExtension");
    }

    #[test]
    fn test_build_option_entry() {
        logger_init();

        assert_eq!(BuildOptionEntry::section_name(), "buildoptions");
        let entry = BuildOptionEntry::new("RELEASE_VS2022_IA32_DLINK_FLAGS".to_string(), None);
        assert_eq!(entry.family, None);
        assert_eq!(entry.target, "RELEASE");
        assert_eq!(entry.tagname, "VS2022");
        assert_eq!(entry.arch, "IA32");
        assert_eq!(entry.tool_code, "DLINK");
        assert_eq!(entry.attribute, "FLAGS");
        assert_eq!(entry.value, "");
        assert_eq!(format!("{}", entry), "RELEASE_VS2022_IA32_DLINK_FLAGS = ");

        let entry = BuildOptionEntry::new(
            "MSFT:RELEASE_*_*_DLINK_PATH".to_string(),
            Some("= C:\\link.exe".to_string()),
        );
        assert_eq!(entry.family, Some("MSFT".to_string()));
        assert_eq!(entry.target, "RELEASE");
        assert_eq!(entry.tagname, "*");
        assert_eq!(entry.arch, "*");
        assert_eq!(entry.tool_code, "DLINK");
        assert_eq!(entry.attribute, "PATH");
        assert_eq!(entry.value, "= C:\\link.exe".to_string());
        assert_eq!(
            format!("{}", entry),
            "MSFT:RELEASE_*_*_DLINK_PATH = = C:\\link.exe"
        );

        let entry = BuildOptionEntry::new(
            "MSFT:*_*_*_CC_FLAGS".to_string(),
            Some("=MyValue".to_string()),
        );
        assert_eq!(entry.family, Some("MSFT".to_string()));
        assert_eq!(entry.target, "*");
        assert_eq!(entry.tagname, "*");
        assert_eq!(entry.arch, "*");
        assert_eq!(entry.tool_code, "CC");
        assert_eq!(entry.attribute, "FLAGS");
        assert_eq!(entry.value, "=MyValue".to_string());
        assert_eq!(format!("{}", entry), "MSFT:*_*_*_CC_FLAGS = =MyValue");

        let entry = BuildOptionEntry::new("MSFT:*_*_*_CC_FLAGS".to_string(), Some("".to_string()));
        assert_eq!(entry.family, Some("MSFT".to_string()));
        assert_eq!(entry.target, "*");
        assert_eq!(entry.tagname, "*");
        assert_eq!(entry.arch, "*");
        assert_eq!(entry.tool_code, "CC");
        assert_eq!(entry.attribute, "FLAGS");
        assert_eq!(entry.value, "".to_string());
        assert_eq!(format!("{}", entry), "MSFT:*_*_*_CC_FLAGS = ");
    }
}
