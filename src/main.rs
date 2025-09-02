use clap::{Parser, Subcommand};
use lib::document::dump;
use lib::model::EnumSetConfig;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pseudo-enum", about = "A rust based tool for generating enums in luau.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    Build {
        #[arg(short = 'c', long)]
        config: Option<PathBuf>,
        #[arg(short = 'o', long)]
        out: Option<PathBuf>,
    },
    Init,
}

pub const DEFAULT_TOML_STR: &str = r#"build_path = "src/Shared/Enums.luau"
use_union_types_for_export = true
use_union_types_for_parameters = true
assign_static_strings = true
[enums]
"TestEnum" = ["Test1", "Test2"]
"#;
pub const DEFAULT_BUILD_PATH: &str = "src/Shared/Enums.luau";
pub const DEFAULT_CONFIG_PATH: &str = "pseudo-enum.toml";

fn main() {
    let args: Args = Args::parse();

    match args.command {
        Some(CliCommand::Build { config, out }) => {
            let config_path = match config {
                Some(path) => path,
                None => PathBuf::from(DEFAULT_CONFIG_PATH),
            };
            let config_content = fs::read_to_string(config_path).expect("Failed to read file");
            let config_set: EnumSetConfig =
                toml::from_str(&config_content).expect("Failed to parse toml");

            let out_path = match out {
                Some(path) => path,
                None => config_set
                    .clone()
                    .build_path
                    .unwrap_or(PathBuf::from(DEFAULT_BUILD_PATH)),
            };
            fs::write(out_path, dump(config_set)).expect("Failed to write to file");
        }
        Some(CliCommand::Init) => {
            fs::write(PathBuf::from(DEFAULT_CONFIG_PATH), DEFAULT_TOML_STR)
                .expect("Failed to write to file");
        }
        None => {
            panic!("No subcommand provided.");
        }
    }
}
