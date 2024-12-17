use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EnumName(String);

impl EnumName {
    pub fn new(input: &str) -> Result<Self, String> {
        if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("EnumValue '{}' has bad character", input));
        }

        Ok(EnumName(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for EnumName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EnumName::new(s)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EnumItem {
    pub name: EnumName,
    pub value: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Enum {
    pub name: EnumName,
    pub items: Vec<EnumItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
enum RawValueData {
    Array(Vec<EnumName>),
    Dictionary(HashMap<EnumName, u16>),
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
        let enum_name = EnumName::new(&key).map_err(serde::de::Error::custom)?;

        let items = match value {
            // If it's an array (e.g. Letters = ["A", "B", "C"])
            toml::Value::Array(arr) => {
                arr.into_iter()
                    .enumerate()
                    .map(|(i, val)| {
                        // Convert each element to a string, then to EnumName
                        let s = val
                            .as_str()
                            .ok_or_else(|| serde::de::Error::custom("Expected string in array"))?;
                        let name = EnumName::new(s).map_err(serde::de::Error::custom)?;
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
                    let name = EnumName::new(&k).map_err(serde::de::Error::custom)?;
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
    #[serde(deserialize_with = "raw_enums_to_enum_set")]
    pub enums: HashSet<Enum>,
}
#[cfg(test)]
pub mod config_test {
    use super::*;
    use toml;

    #[test]
    fn parse_config() {
        let toml_str = r#"
			use_union_types_for_parameters = true
			use_union_types_for_export = true
			[enums]
			Letters = ["A", "B", "C"]

			[enums.Device]
			Phone = 100
			Tablet = 250
			Console = 300
		"#;
        let config: EnumSetConfig = toml::from_str(toml_str).expect("Failed to parse toml");
        println!("config {:#?}", config)
    }
}
