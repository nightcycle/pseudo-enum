use clap::{Parser, Subcommand};
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
          config: PathBuf,
          #[arg(short = 'o', long)]
          out: Option<PathBuf>,
     },
     Init,
}

async fn main() {
     let args: Args = Args::parse();

     match args.command {
          Some(CliCommand::Build { config, out }) => {}
          Some(CliCommand::Init) => {}
          None => {
               panic!("No subcommand provided.");
          }
     }
}
