use crate::model::{Enum, EnumSetConfig};
use serde::{Deserialize, Serialize};
use std::fmt;
use stylua_lib::{
    self, CallParenType, CollapseSimpleStatement, Config, IndentType, LineEndings,
    OutputVerification, QuoteStyle, SortRequiresConfig,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EnumTypeDefinition {
    pub value: Enum,
    pub is_exported: bool,
}

impl fmt::Display for EnumTypeDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut union_str = String::new();
        for (i, item) in self.value.items.iter().enumerate() {
            if i > 0 {
                union_str.push_str(" | ");
            }
            union_str.push_str(&format!("\"{}\"", item.name));
        }
        if self.is_exported {
            write!(f, "export type {} = {}", self.value.name, union_str)
        } else {
            write!(f, "type {} = {}", self.value.name, union_str)
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EnumListDefinition {
    pub value: Enum,
    pub is_frozen: bool,
}

impl EnumListDefinition {
    pub fn get_variable_name(&self) -> String {
        return format!("{}List", self.value.name);
    }
}

impl fmt::Display for EnumListDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut union_str = String::new();
        for (i, item) in self.value.items.iter().enumerate() {
            if i > 0 {
                union_str.push_str(",");
            }
            union_str.push_str(&format!("\"{}\"", item.name));
        }
        if self.is_frozen {
            write!(
                f,
                "local {} = table.freeze({{{}}})",
                self.get_variable_name(),
                union_str
            )
        } else {
            write!(f, "local {} = {{{}}}", self.get_variable_name(), union_str)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EnumDictDefinition {
    pub value: Enum,
    pub is_frozen: bool,
    pub assign_as_static_string: bool,
}

impl EnumDictDefinition {
    pub fn get_variable_name(&self) -> String {
        return format!("{}Dict", self.value.name);
    }
}

impl fmt::Display for EnumDictDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut union_str = String::new();
        for (i, item) in self.value.items.iter().enumerate() {
            if i > 0 {
                union_str.push_str(",\n");
            } else {
                union_str.push_str("\n");
            }
            if self.assign_as_static_string {
                union_str.push_str(&format!(
                    "\t{}=\"{}\" :: {}",
                    item.name,
                    item.name,
                    format!("\"{}\"", item.name)
                ));
            } else {
                union_str.push_str(&format!(
                    "\t{}=\"{}\" :: {}",
                    item.name, item.name, self.value.name
                ));
            }
        }
        if self.is_frozen {
            write!(
                f,
                "local {} = table.freeze({{{}\n}})",
                self.get_variable_name(),
                union_str
            )
        } else {
            write!(
                f,
                "local {} = {{{}\n}}",
                self.get_variable_name(),
                union_str
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EnumValueDictDefinition {
    pub value: Enum,
    pub is_frozen: bool,
}

impl EnumValueDictDefinition {
    pub fn get_variable_name(&self) -> String {
        return format!("{}ValueDict", self.value.name);
    }
}

impl fmt::Display for EnumValueDictDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut union_str: String = String::new();
        for (i, item) in self.value.items.iter().enumerate() {
            if i > 0 {
                union_str.push_str(",\n");
            } else {
                union_str.push_str("\n");
            }
            union_str.push_str(&format!("\t{}={}", item.name, item.value,));
        }
        if self.is_frozen {
            write!(
                f,
                "local {} = table.freeze({{{}\n}})",
                self.get_variable_name(),
                union_str
            )
        } else {
            write!(
                f,
                "local {} = {{{}\n}}",
                self.get_variable_name(),
                union_str
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EnumInverseValueDictDefinition {
    pub value: Enum,
    pub is_frozen: bool,
    pub assign_as_static_string: bool,
}

impl EnumInverseValueDictDefinition {
    pub fn get_variable_name(&self) -> String {
        return format!("{}InverseValueDict", self.value.name);
    }
}

impl fmt::Display for EnumInverseValueDictDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut union_str: String = String::new();
        for (i, item) in self.value.items.iter().enumerate() {
            if i > 0 {
                union_str.push_str(",\n");
            } else {
                union_str.push_str("\n");
            }
            if self.assign_as_static_string {
                union_str.push_str(&format!("\t[{}]=\"{}\"", item.value, item.name));
            } else {
                union_str.push_str(&format!(
                    "\t[{}]=\"{}\" :: {}",
                    item.value, item.name, self.value.name
                ));
            }
        }
        if self.is_frozen {
            write!(
                f,
                "local {} = table.freeze({{{}\n}})",
                self.get_variable_name(),
                union_str
            )
        } else {
            write!(
                f,
                "local {} = {{{}\n}}",
                self.get_variable_name(),
                union_str
            )
        }
    }
}

#[allow(deprecated)]
fn format_code(code: String) -> String {
    let style_result: Result<String, stylua_lib::Error> = stylua_lib::format_code(
        &code,
        Config {
            column_width: 200,
            line_endings: LineEndings::Windows,
            indent_type: IndentType::Tabs,
            indent_width: 5,
            quote_style: QuoteStyle::AutoPreferDouble,
            no_call_parentheses: false,
            call_parentheses: CallParenType::Always,
            collapse_simple_statement: CollapseSimpleStatement::ConditionalOnly,
            sort_requires: SortRequiresConfig::new(),
            space_after_function_names: stylua_lib::SpaceAfterFunctionNames::Never,
            syntax: stylua_lib::LuaVersion::Luau,
        },
        Option::None,
        OutputVerification::None,
    );

    let fmt_content = match style_result {
        Ok(out) => out,
        Err(error) => {
            panic!("Problem styling code: {}, \n{}", error, code)
        }
    };

    return fmt_content;
    // return code
}

pub fn dump(config: EnumSetConfig) -> String {
    let mut header = String::new();
    header.push_str("--!strict");
    header.push_str("\n-- DO NOT EDIT MANUALLY!! This file was generated by nightcycle/pseudo-enum, edits will likely be overwritten!");

    for e in config.enums.iter() {
        header.push_str(&format!(
            "\n{}",
            EnumTypeDefinition {
                value: e.clone(),
                is_exported: true,
            }
        ));
        header.push_str(&format!(
            "\n{}",
            EnumListDefinition {
                value: e.clone(),
                is_frozen: true,
            }
        ));
        header.push_str(&format!(
            "\n{}",
            EnumDictDefinition {
                value: e.clone(),
                is_frozen: true,
                assign_as_static_string: config.assign_static_strings
            }
        ));
        header.push_str(&format!(
            "\n{}",
            EnumValueDictDefinition {
                value: e.clone(),
                is_frozen: true
            }
        ));
        header.push_str(&format!(
            "\n{}",
            EnumInverseValueDictDefinition {
                value: e.clone(),
                is_frozen: true,
                assign_as_static_string: config.assign_static_strings
            }
        ));
    }

    let mut list_tree = String::new();
    list_tree.push_str("\nlocal listTree = {");
    for e in config.enums.iter() {
        list_tree.push_str(&format!(
            "\n\t{} = {},",
            e.name,
            EnumListDefinition {
                value: e.clone(),
                is_frozen: true,
            }
            .get_variable_name(),
        ))
    }
    list_tree.push_str("\n}");
    list_tree.push_str("\ntable.freeze(listTree)");

    let mut value_tree = String::new();
    value_tree.push_str("\nlocal valueTree = {");
    for e in config.enums.iter() {
        value_tree.push_str(&format!(
            "\n\t{} = {},",
            e.name,
            EnumValueDictDefinition {
                value: e.clone(),
                is_frozen: true,
            }
            .get_variable_name(),
        ))
    }
    value_tree.push_str("\n}");
    value_tree.push_str("\ntable.freeze(valueTree)");

    let mut inv_value_tree = String::new();
    inv_value_tree.push_str("\nlocal invValueTree = {");
    for e in config.enums.iter() {
        inv_value_tree.push_str(&format!(
            "\n\t{} = {},",
            e.name,
            EnumInverseValueDictDefinition {
                value: e.clone(),
                is_frozen: true,
                assign_as_static_string: config.assign_static_strings
            }
            .get_variable_name(),
        ))
    }
    inv_value_tree.push_str("\n}");
    inv_value_tree.push_str("\ntable.freeze(invValueTree)");

    let mut enum_name_type = String::new();
    {
        enum_name_type.push_str("\nexport type EnumName = ");
        for (i, e) in config.enums.iter().enumerate() {
            if i > 0 {
                enum_name_type.push_str(" | ");
            }
            enum_name_type.push_str(&format!("\"{}\"", e.name));
        }
    }

    let mut get_name_union_type = String::new();
    {
        get_name_union_type.push_str("(");
        for (i, e) in config.enums.iter().enumerate() {
            if i > 0 {
                get_name_union_type.push_str(" & ");
            }
            get_name_union_type.push_str(&format!("((\"{}\", number) -> {})", e.name, e.name));
        }
        get_name_union_type.push_str(")");
    }

    let mut get_value_union_type = String::new();
    {
        get_value_union_type.push_str("(");
        for (i, e) in config.enums.iter().enumerate() {
            if i > 0 {
                get_value_union_type.push_str(" & ");
            }
            get_value_union_type.push_str(&format!("((\"{}\", {}) -> number)", e.name, e.name));
        }
        get_value_union_type.push_str(")");
    }

    let mut get_items_union_type = String::new();
    {
        get_items_union_type.push_str("(");
        for (i, e) in config.enums.iter().enumerate() {
            if i > 0 {
                get_items_union_type.push_str(" & ");
            }
            get_items_union_type.push_str(&format!("((\"{}\") -> {{{}}})", e.name, e.name));
        }
        get_items_union_type.push_str(")");
    }

    let mut interface = String::new();
    interface.push_str("\nreturn {");
    interface.push_str("\n\tgetEnumItems = function(enumName: EnumName)");
    interface.push_str("\n\t\tlocal list = listTree[enumName]");
    interface.push_str("\n\t\tassert(list, `invalid enumName: \"{enumName}\"`)");
    interface.push_str("\n\t\treturn list");
    interface.push_str(&format!("\n\tend :: {},", get_items_union_type));
    interface.push_str("\n\tgetEnumItemFromValue = function(enumName: EnumName, value: number)");
    interface.push_str("\n\t\tlocal dict = invValueTree[enumName]");
    interface.push_str("\n\t\tassert(dict, `invalid enumName: \"{enumName}\"`)");
    interface.push_str("\n\t\tlocal name = dict[value]");
    interface.push_str("\n\t\tassert(name, `invalid value: \"{enumName}\" -> {value}`)");
    interface.push_str("\n\t\treturn name");
    interface.push_str(&format!("\n\tend :: {},", get_name_union_type));
    interface
        .push_str("\n\tgetValueFromEnumItem = function(enumName: EnumName, name: string): number");
    interface.push_str("\n\t\tlocal dict = valueTree[enumName]");
    interface.push_str("\n\t\tassert(dict, `invalid enumName: \"{enumName}\"`)");
    interface.push_str("\n\t\tlocal value = dict[name]");
    interface.push_str("\n\t\tassert(value, `invalid value: \"{enumName}\" -> \"{name}\"`)");
    interface.push_str("\n\t\treturn value");
    interface.push_str(&format!("\n\tend :: {},", get_value_union_type));

    for e in config.enums.iter() {
        interface.push_str(&format!(
            "\n\t{} = {},",
            e.name,
            EnumDictDefinition {
                value: e.clone(),
                is_frozen: true,
                assign_as_static_string: config.assign_static_strings
            }
            .get_variable_name(),
        ))
    }

    interface.push_str("\n}");

    let mut code = String::new();
    code.push_str(&format!(
        "{}{}{}{}{}{}",
        header, list_tree, value_tree, inv_value_tree, enum_name_type, interface
    ));

    code = format_code(code);
    return code;
}

#[cfg(test)]
pub mod config_test {
    use super::*;
    use toml;

    #[test]
    fn parse_config() {
        let config: EnumSetConfig =
            toml::from_str(crate::model::config_test::TOML_STR).expect("Failed to parse toml");
        println!("result:\n{}", dump(config));
    }
}
