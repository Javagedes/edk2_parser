use crate::err::ParseError;
use anyhow::Result;
use rmp_serde as rmps;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

pub enum SectionType {
    Defines,
    Sources,
    Binaries,
    Protocols,
    Ppis,
    Guids,
    LibraryClasses,
    Packages,
    FeaturePcd,
    FixedPcd,
    PatchPcd,
    Pcd,
    PcdEx,
    Depex,
    UserExtensions,
    BuildOptions,
}

impl SectionType {
    pub fn to_bytes(&self, key: String, value: Option<String>) -> Result<Vec<u8>> {
        match self {
            SectionType::Defines => DefineEntry::to_bytes(key, value),
            SectionType::Sources => SourceEntry::to_bytes(key, value),
            SectionType::Binaries => BinaryEntry::to_bytes(key, value),
            SectionType::Protocols => ProtocolEntry::to_bytes(key, value),
            SectionType::Ppis => PpiEntry::to_bytes(key, value),
            SectionType::Guids => GuidEntry::to_bytes(key, value),
            SectionType::LibraryClasses => LibraryClassEntry::to_bytes(key, value),
            SectionType::Packages => PackageEntry::to_bytes(key, value),
            SectionType::FeaturePcd => FeaturePcdEntry::to_bytes(key, value),
            SectionType::FixedPcd => FixedPcdEntry::to_bytes(key, value),
            SectionType::PatchPcd => PatchPcdEntry::to_bytes(key, value),
            SectionType::Pcd => PcdEntry::to_bytes(key, value),
            SectionType::PcdEx => PcdExEntry::to_bytes(key, value),
            SectionType::Depex => DepexEntry::to_bytes(key, value),
            SectionType::UserExtensions => UserExtensionEntry::to_bytes(key, value),
            SectionType::BuildOptions => BuildOptionEntry::to_bytes(key, value),
        }
    }
}

impl FromStr for SectionType {
    type Err = ParseError;

    fn from_str(section_name: &str) -> Result<Self, Self::Err> {
        match section_name {
            "defines" => Ok(SectionType::Defines),
            "sources" => Ok(SectionType::Sources),
            "binaries" => Ok(SectionType::Binaries),
            "protocols" => Ok(SectionType::Protocols),
            "ppis" => Ok(SectionType::Ppis),
            "guids" => Ok(SectionType::Guids),
            "libraryclasses" => Ok(SectionType::LibraryClasses),
            "packages" => Ok(SectionType::Packages),
            "featurepcd" => Ok(SectionType::FeaturePcd),
            "fixedpcd" => Ok(SectionType::FixedPcd),
            "patchpcd" => Ok(SectionType::PatchPcd),
            "pcd" => Ok(SectionType::Pcd),
            "pcdex" => Ok(SectionType::PcdEx),
            "depex" => Ok(SectionType::Depex),
            "userextensions" => Ok(SectionType::UserExtensions),
            "buildoptions" => Ok(SectionType::BuildOptions),
            _ => Err(ParseError::UnknownSection(section_name.to_string())),
        }
    }
}

pub trait Edk2SectionEntry: Display + Serialize + Deserialize<'static> {
    fn section_name() -> &'static str
    where
        Self: Sized;

    fn from_key_value_pair(key: String, value: Option<String>) -> Result<Self>;

    fn to_bytes(key: String, value: Option<String>) -> Result<Vec<u8>> {
        let section_entry = Self::from_key_value_pair(key, value)?;
        let mut buf = Vec::new();
        section_entry.serialize(&mut rmps::Serializer::new(&mut buf))?;
        Ok(buf)
    }

    fn from_bytes(buf: &[u8]) -> Result<Self> {
        let mut de = rmps::Deserializer::new(buf);
        let section_entry = Self::deserialize(&mut de)?;
        Ok(section_entry)
    }
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
#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, value: Option<String>) -> Result<Self> {
        Ok(Self {
            name: key.trim().to_string(),
            value: value
                .ok_or(ParseError::InvalidFormat("<name> = <value>".to_string()))?
                .trim()
                .to_string(),
        })
    }
}

/// Contains the parsed data from a single entry in the [Sources] section of an INF file.
///
/// # Define line format
///
/// ```text
///   path
/// ```
#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();

        let (path, family) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(parts[1])),
            _ => return Err(ParseError::InvalidFormat("<path>[|<family>]".to_string()).into()),
        };

        Ok(Self {
            path: path.trim().to_string(),
            family: family.map(|f| f.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();

        let (filetype, path, target, family, tagname) = match parts.len() {
            2 => (parts[0], parts[1], "*", "*", "*"),
            3 => (parts[0], parts[1], parts[2], "*", "*"),
            4 => (parts[0], parts[1], parts[2], parts[3], "*"),
            5 => (parts[0], parts[1], parts[2], parts[3], parts[4]),
            _ => {
                return Err(ParseError::InvalidFormat(
                    "<filetype>|<path>[|<target>[|<family>[|<tagname>]]]".to_string(),
                )
                .into())
            }
        };

        Ok(Self {
            filetype: filetype.trim().to_string(),
            path: path.trim().to_string(),
            target: target.trim().to_string(),
            family: family.trim().to_string(),
            tagname: tagname.trim().to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();

        let (name, expression) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(parts[1])),
            _ => return Err(ParseError::InvalidFormat("<name>[|<expression>]".to_string()).into()),
        };

        Ok(Self {
            name: name.trim().to_string(),
            expression: expression.map(|f| f.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();

        let (name, expression) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(parts[1])),
            _ => return Err(ParseError::InvalidFormat("<name>[|<expression>]".to_string()).into()),
        };

        Ok(Self {
            name: name.trim().to_string(),
            expression: expression.map(|f| f.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();

        let (name, expression) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(parts[1])),
            _ => return Err(ParseError::InvalidFormat("<name>[|<expression>]".to_string()).into()),
        };

        Ok(Self {
            name: name.trim().to_string(),
            expression: expression.map(|x| x.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();

        let (name, expression) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(parts[1])),
            _ => return Err(ParseError::InvalidFormat("<name>[|<expression>]".to_string()).into()),
        };

        Ok(Self {
            name: name.trim().to_string(),
            expression: expression.map(|x| x.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        Ok(Self {
            name: key.trim().to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();
        let err = "<token_space>.<name>[|<value>][|<expression>]".to_string();

        let (token_space, name, value, expression) = match parts.len() {
            1 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, None, None)
            }
            2 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), None)
            }
            3 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), Some(parts[2]))
            }
            _ => return Err(ParseError::InvalidFormat(err).into()),
        };

        Ok(Self {
            token_space: token_space.trim().to_string(),
            name: name.trim().to_string(),
            value: value.map(|x| x.trim().to_string()),
            expression: expression.map(|x| x.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();
        let err = "<token_space>.<name>[|<value>][|<expression>]".to_string();

        let (token_space, name, value, expression) = match parts.len() {
            1 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, None, None)
            }
            2 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), None)
            }
            3 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), Some(parts[2]))
            }
            _ => return Err(ParseError::InvalidFormat(err).into()),
        };

        Ok(Self {
            token_space: token_space.trim().to_string(),
            name: name.trim().to_string(),
            value: value.map(|x| x.trim().to_string()),
            expression: expression.map(|x| x.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();
        let err = "PatchPcdEntry must be in format <token_space>.<name>|<value>|<hex>".to_string();
        let (token_space, name, value, hex) = match parts.len() {
            3 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, parts[1], parts[2])
            }
            _ => return Err(ParseError::InvalidFormat(err).into()),
        };

        Ok(Self {
            token_space: token_space.trim().to_string(),
            name: name.trim().to_string(),
            value: value.trim().to_string(),
            hex: hex.trim().to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();
        let err = "<token_space>.<name>[|<value>][|<expression>]".to_string();
        let (token_space, name, value, expression) = match parts.len() {
            1 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, None, None)
            }
            2 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), None)
            }
            3 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), Some(parts[2]))
            }
            _ => return Err(ParseError::InvalidFormat(err).into()),
        };

        Ok(Self {
            token_space: token_space.trim().to_string(),
            name: name.trim().to_string(),
            value: value.map(|x| x.trim().to_string()),
            expression: expression.map(|x| x.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PcdExEntry {
    pub token_space: String,
    pub name: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}
impl Display for PcdExEntry {
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
impl Edk2SectionEntry for PcdExEntry {
    fn section_name() -> &'static str {
        "pcdex"
    }
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = key.split('|').collect();
        let err = "<token_space>.<name>[|<value>][|<expression>]".to_string();

        let (token_space, name, value, expression) = match parts.len() {
            1 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, None, None)
            }
            2 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), None)
            }
            3 => {
                let (name, value) = parts[0]
                    .split_once('.')
                    .ok_or(ParseError::InvalidFormat(err))?;
                (name, value, Some(parts[1]), Some(parts[2]))
            }
            _ => return Err(ParseError::InvalidFormat(err).into()),
        };

        Ok(Self {
            token_space: token_space.trim().to_string(),
            name: name.trim().to_string(),
            value: value.map(|x| x.trim().to_string()),
            expression: expression.map(|x| x.trim().to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        Ok(Self {
            value: key.trim().to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, _value: Option<String>) -> Result<Self> {
        Ok(Self {
            value: key.trim().to_string(),
        })
    }
}

/// [<family>:]<target>_<tagname>_<arch>_<tool_code>_<attribute> = <value>
/// if value starts with a "=", replace the entry if it exists.
/// otherwise, just append.
/// If a $() is in quotes, don't replace TODO: replace_macro probably breaks this
#[derive(Debug, Serialize, Deserialize)]
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
    fn from_key_value_pair(key: String, value: Option<String>) -> Result<Self> {
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
                return Err(ParseError::InvalidFormat(
                    "[<family>:]<target>_<tagname>_<arch>_<tool_code>_<attribute> = <value>"
                        .to_string(),
                )
                .into())
            }
        };

        Ok(Self {
            family: family.map(|x| x.trim().to_string()),
            target: target.trim().to_string(),
            tagname: tagname.trim().to_string(),
            arch: arch.trim().to_string(),
            tool_code: tool_code.trim().to_string(),
            attribute: attribute.trim().to_string(),
            value: value.unwrap_or("".to_string()),
        })
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
        let entry = DefineEntry::from_key_value_pair(
            "INF_VERSION".to_string(),
            Some(" 0x00010005".to_string()),
        )
        .unwrap();
        assert_eq!(entry.name, "INF_VERSION");
        assert_eq!(entry.value, "0x00010005");
        assert_eq!(format!("{}", entry), "INF_VERSION = 0x00010005");
    }

    #[test]
    fn test_source_entry() {
        logger_init();

        assert_eq!(SourceEntry::section_name(), "sources");
        let entry = SourceEntry::from_key_value_pair("  MyFile.c".to_string(), None).unwrap();
        assert_eq!(entry.path, "MyFile.c");
        assert_eq!(format!("{}", entry), "MyFile.c");

        let entry = SourceEntry::from_key_value_pair("MyFile.c | MSFT".to_string(), None).unwrap();
        assert_eq!(entry.path, "MyFile.c");
        assert_eq!(entry.family, Some("MSFT".to_string()));
        assert_eq!(format!("{}", entry), "MyFile.c|MSFT");
    }

    #[test]
    fn test_binary_entry() {
        logger_init();

        assert_eq!(BinaryEntry::section_name(), "binaries");
        let entry =
            BinaryEntry::from_key_value_pair("RAW|MyFile/file.ext".to_string(), None).unwrap();
        assert_eq!(entry.filetype, "RAW");
        assert_eq!(entry.path, "MyFile/file.ext");
        assert_eq!(entry.target, "*");
        assert_eq!(entry.family, "*");
        assert_eq!(entry.tagname, "*");
        assert_eq!(format!("{}", entry), "RAW|MyFile/file.ext|*|*|*");

        let entry =
            BinaryEntry::from_key_value_pair("RAW|MyFile/file.ext|*|MSFT".to_string(), None)
                .unwrap();
        assert_eq!(entry.filetype, "RAW");
        assert_eq!(entry.path, "MyFile/file.ext");
        assert_eq!(entry.target, "*");
        assert_eq!(entry.family, "MSFT");
        assert_eq!(entry.tagname, "*");
        assert_eq!(format!("{}", entry), "RAW|MyFile/file.ext|*|MSFT|*");

        let entry = BinaryEntry::from_key_value_pair(
            "RAW|MyFile/file.ext|RELEASE|*|MyTag".to_string(),
            None,
        )
        .unwrap();
        assert_eq!(entry.filetype, "RAW");
        assert_eq!(entry.path, "MyFile/file.ext");
        assert_eq!(entry.target, "RELEASE");
        assert_eq!(entry.family, "*");
        assert_eq!(entry.tagname, "MyTag");
        assert_eq!(format!("{}", entry), "RAW|MyFile/file.ext|RELEASE|*|MyTag");

        let entry =
            BinaryEntry::from_key_value_pair("RAW|MyFile/file.ext|*|*|MyTag".to_string(), None)
                .unwrap();
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
        let entry =
            ProtocolEntry::from_key_value_pair("gEfiPciIoProtocolGuid".to_string(), None).unwrap();
        assert_eq!(entry.name, "gEfiPciIoProtocolGuid");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gEfiPciIoProtocolGuid");

        let entry =
            ProtocolEntry::from_key_value_pair("gEfiPciIoProtocolGuid|TRUE".to_string(), None)
                .unwrap();
        assert_eq!(entry.name, "gEfiPciIoProtocolGuid");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gEfiPciIoProtocolGuid | TRUE");
    }

    #[test]
    fn test_ppi_entry() {
        logger_init();

        assert_eq!(PpiEntry::section_name(), "ppis");
        let entry =
            PpiEntry::from_key_value_pair("gEfiPeiMemoryDiscoveredPpiGuid".to_string(), None)
                .unwrap();
        assert_eq!(entry.name, "gEfiPeiMemoryDiscoveredPpiGuid");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gEfiPeiMemoryDiscoveredPpiGuid");

        let entry =
            PpiEntry::from_key_value_pair("gEfiPeiMemoryDiscoveredPpiGuid|TRUE".to_string(), None)
                .unwrap();
        assert_eq!(entry.name, "gEfiPeiMemoryDiscoveredPpiGuid");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gEfiPeiMemoryDiscoveredPpiGuid|TRUE");
    }

    #[test]
    fn test_guid_entry() {
        logger_init();

        assert_eq!(GuidEntry::section_name(), "guids");
        let entry =
            GuidEntry::from_key_value_pair("gEfiHobMemoryAllocModuleGuid".to_string(), None)
                .unwrap();
        assert_eq!(entry.name, "gEfiHobMemoryAllocModuleGuid");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gEfiHobMemoryAllocModuleGuid");

        let entry =
            GuidEntry::from_key_value_pair("gEfiHobMemoryAllocModuleGuid|TRUE".to_string(), None)
                .unwrap();
        assert_eq!(entry.name, "gEfiHobMemoryAllocModuleGuid");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gEfiHobMemoryAllocModuleGuid|TRUE");
    }

    #[test]
    fn test_library_class_entry() {
        logger_init();

        assert_eq!(LibraryClassEntry::section_name(), "libraryclasses");
        let entry = LibraryClassEntry::from_key_value_pair("MyLib".to_string(), None).unwrap();
        assert_eq!(entry.name, "MyLib");
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "MyLib");

        let entry = LibraryClassEntry::from_key_value_pair("MyLib|TRUE".to_string(), None).unwrap();
        assert_eq!(entry.name, "MyLib");
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "MyLib|TRUE");
    }

    #[test]
    fn test_package_entry() {
        logger_init();

        assert_eq!(PackageEntry::section_name(), "packages");
        let entry =
            PackageEntry::from_key_value_pair("MdePkg/MdePkg.dec".to_string(), None).unwrap();
        assert_eq!(entry.name, "MdePkg/MdePkg.dec");
        assert_eq!(format!("{}", entry), "MdePkg/MdePkg.dec");
    }

    #[test]
    fn test_feature_pcd_entry() {
        logger_init();

        assert_eq!(FeaturePcdEntry::section_name(), "featurepcd");
        let entry =
            FeaturePcdEntry::from_key_value_pair("gTokenSpace.PcdMyFeature|TRUE".to_string(), None)
                .unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature|TRUE");

        let entry =
            FeaturePcdEntry::from_key_value_pair("gTokenSpace.PcdMyFeature".to_string(), None)
                .unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature");

        let entry = FeaturePcdEntry::from_key_value_pair(
            "gTokenSpace.PcdMyFeature|L\"HELLO\"|TRUE".to_string(),
            None,
        )
        .unwrap();
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
        let entry =
            FixedPcdEntry::from_key_value_pair("gTokenSpace.PcdMyFeature|TRUE".to_string(), None)
                .unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature|TRUE");

        let entry =
            FixedPcdEntry::from_key_value_pair("gTokenSpace.PcdMyFeature".to_string(), None)
                .unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "PcdMyFeature");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.PcdMyFeature");

        let entry = FixedPcdEntry::from_key_value_pair(
            "gTokenSpace.PcdMyFeature|L\"HELLO\"|TRUE".to_string(),
            None,
        )
        .unwrap();
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
        let entry = PatchPcdEntry::from_key_value_pair(
            "gTokenSpace.PcdMyFeature|TRUE|0x1234".to_string(),
            None,
        )
        .unwrap();
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
        let entry =
            PcdEntry::from_key_value_pair("gTokenSpace.MyPcd|TRUE".to_string(), None).unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd|TRUE");

        let entry = PcdEntry::from_key_value_pair("gTokenSpace.MyPcd".to_string(), None).unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd");

        let entry =
            PcdEntry::from_key_value_pair("gTokenSpace.MyPcd|L\"HELLO\"|TRUE".to_string(), None)
                .unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, Some("L\"HELLO\"".to_string()));
        assert_eq!(entry.expression, Some("TRUE".to_string()));
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd|L\"HELLO\"|TRUE");
    }

    #[test]
    fn test_pcd_ex_entry() {
        logger_init();

        assert_eq!(PcdExEntry::section_name(), "pcdex");
        let entry =
            PcdExEntry::from_key_value_pair("gTokenSpace.MyPcd|TRUE".to_string(), None).unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, Some("TRUE".to_string()));
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd|TRUE");

        let entry = PcdExEntry::from_key_value_pair("gTokenSpace.MyPcd".to_string(), None).unwrap();
        assert_eq!(entry.token_space, "gTokenSpace");
        assert_eq!(entry.name, "MyPcd");
        assert_eq!(entry.value, None);
        assert_eq!(entry.expression, None);
        assert_eq!(format!("{}", entry), "gTokenSpace.MyPcd");

        let entry =
            PcdExEntry::from_key_value_pair("gTokenSpace.MyPcd|L\"HELLO\"|TRUE".to_string(), None)
                .unwrap();
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
        let entry = DepexEntry::from_key_value_pair("TRUE".to_string(), None).unwrap();
        assert_eq!(entry.value, "TRUE");
        assert_eq!(format!("{}", entry), "TRUE");
    }

    #[test]
    fn test_user_extension_entry() {
        logger_init();

        assert_eq!(UserExtensionEntry::section_name(), "userextensions");
        let entry =
            UserExtensionEntry::from_key_value_pair("MyUserExtension".to_string(), None).unwrap();
        assert_eq!(entry.value, "MyUserExtension");
        assert_eq!(format!("{}", entry), "MyUserExtension");
    }

    #[test]
    fn test_build_option_entry() {
        logger_init();

        assert_eq!(BuildOptionEntry::section_name(), "buildoptions");
        let entry = BuildOptionEntry::from_key_value_pair(
            "RELEASE_VS2022_IA32_DLINK_FLAGS".to_string(),
            None,
        )
        .unwrap();
        assert_eq!(entry.family, None);
        assert_eq!(entry.target, "RELEASE");
        assert_eq!(entry.tagname, "VS2022");
        assert_eq!(entry.arch, "IA32");
        assert_eq!(entry.tool_code, "DLINK");
        assert_eq!(entry.attribute, "FLAGS");
        assert_eq!(entry.value, "");
        assert_eq!(format!("{}", entry), "RELEASE_VS2022_IA32_DLINK_FLAGS = ");

        let entry = BuildOptionEntry::from_key_value_pair(
            "MSFT:RELEASE_*_*_DLINK_PATH".to_string(),
            Some("= C:\\link.exe".to_string()),
        )
        .unwrap();
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

        let entry = BuildOptionEntry::from_key_value_pair(
            "MSFT:*_*_*_CC_FLAGS".to_string(),
            Some("=MyValue".to_string()),
        )
        .unwrap();
        assert_eq!(entry.family, Some("MSFT".to_string()));
        assert_eq!(entry.target, "*");
        assert_eq!(entry.tagname, "*");
        assert_eq!(entry.arch, "*");
        assert_eq!(entry.tool_code, "CC");
        assert_eq!(entry.attribute, "FLAGS");
        assert_eq!(entry.value, "=MyValue".to_string());
        assert_eq!(format!("{}", entry), "MSFT:*_*_*_CC_FLAGS = =MyValue");

        let entry = BuildOptionEntry::from_key_value_pair(
            "MSFT:*_*_*_CC_FLAGS".to_string(),
            Some("".to_string()),
        )
        .unwrap();
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
