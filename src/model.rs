use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LuauName(String);

impl LuauName {
    pub fn new(input: &str) -> Result<Self, String> {
        if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("EnumValue '{}' has bad character", input));
        }

        Ok(LuauName(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for LuauName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LuauName::new(s)
    }
}

impl fmt::Display for LuauName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EnumItem {
    pub name: LuauName,
    pub value: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Enum {
    pub name: LuauName,
    pub items: Vec<EnumItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
enum RawValueData {
    Array(Vec<LuauName>),
    Dictionary(HashMap<LuauName, u16>),
}

#[derive(Debug, Deserialize)]
struct EnumsTable {
    #[serde(flatten)]
    enums: HashMap<String, toml::Value>,
}

fn raw_enums_to_enum_set<'de, D>(deser: D) -> Result<HashSet<Enum>, D::Error>
where
    D: Deserializer<'de>,
{
    // First, parse `[enums]` into a HashMap of `String -> toml::Value`.
    let table = EnumsTable::deserialize(deser)?;

    let mut enums = HashSet::new();

    for (key, value) in table.enums {
        let enum_name = LuauName::new(&key).map_err(serde::de::Error::custom)?;

        let items = match value {
            // If it's an array (e.g. Letters = ["A", "B", "C"])
            toml::Value::Array(arr) => {
                arr.into_iter()
                    .enumerate()
                    .map(|(i, val)| {
                        // Convert each element to a string, then to LuauName
                        let s = val
                            .as_str()
                            .ok_or_else(|| serde::de::Error::custom("Expected string in array"))?;
                        let name = LuauName::new(s).map_err(serde::de::Error::custom)?;
                        Ok(EnumItem {
                            name,
                            value: (i as u16) + 1,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?
            }

            // If it's a table (e.g. [enums.Device] subtable)
            toml::Value::Table(tbl) => {
                let mut items = Vec::new();
                for (k, v) in tbl {
                    let name = LuauName::new(&k).map_err(serde::de::Error::custom)?;
                    let value = v
                        .as_integer()
                        .ok_or_else(|| serde::de::Error::custom("Expected integer in table"))?;
                    items.push(EnumItem {
                        name,
                        value: value as u16,
                    });
                }
                // Sort items by their `value`
                items.sort_by_key(|item| item.value);
                items
            }

            // Otherwise, unexpected type
            _ => {
                return Err(serde::de::Error::custom(
                    "Expected array or table for enum definition",
                ));
            }
        };

        enums.insert(Enum {
            name: enum_name,
            items,
        });
    }

    Ok(enums)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnumSetConfig {
    pub build_path: Option<PathBuf>,
    pub use_union_types_for_export: bool,
    pub use_union_types_for_parameters: bool,
    pub assign_static_strings: bool,
    #[serde(deserialize_with = "raw_enums_to_enum_set")]
    pub enums: HashSet<Enum>,
}
#[cfg(test)]
pub mod config_test {
    use super::*;
    use toml;

    pub const TOML_STR: &str = r#"
build_path = "src/Shared/Enums.luau"
use_union_types_for_export = true
use_union_types_for_parameters = true
assign_static_strings = true
[enums]
MapType = ["City", "PowerLab"]
RunMode = ["Dev", "Live"]
EffectClassId = ["None", "Grow", "Forcefield"]
MoveClassId = ["None", "Flight", "SuperSpeed"]
EquipContext = ["Right", "Left", "Dual"]
ToolClassId = ["BladeCutlass"]
ToolEvent = ["Primary", "Secondary"]
	"#;

    #[test]
    fn deserialize_config() {
        let config: EnumSetConfig = toml::from_str(TOML_STR).expect("Failed to parse toml");
        println!("config {:#?}", config)
    }

    #[test]
    fn serialize_config() {
        let config: EnumSetConfig = toml::from_str(TOML_STR).expect("Failed to parse toml");
        let out = toml::to_string(&config).expect("Failed to serialize config");
        println!("config {}", out)
    }
}
