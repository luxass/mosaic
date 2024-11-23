use std::mem;

use anyhow::Result;
use clap::{
  builder::NonEmptyStringValueParser, ArgAction, ArgGroup, CommandFactory, Parser, Subcommand,
  ValueEnum,
};

#[derive(Parser, Clone, Default, Debug, PartialEq)]
#[clap(author, about = "An API for your projects", long_about = None)]
#[clap(disable_help_subcommand = true)]
#[clap(arg_required_else_help = true)]
#[command(name = "mosaic")]
pub struct Args {
  #[clap(subcommand)]
  pub command: Option<Command>,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Command {
  Run,
  Validate,
  Query,
}

#[tokio::main]
async fn main() -> Result<()> {
  let mut cli_args = Args::parse();
  let mut command = if let Some(command) = mem::take(&mut cli_args.command) {
    command
  } else {
    Command::Run
  };

  cli_args.command = Some(command);

  // TODO: set cwd

  let cli_result = match cli_args.command.as_ref().unwrap() {
    Command::Run => {
      println!("Running Mosaic...");
      Ok(())
    },
    Command::Validate => {
      todo!()
    },
    Command::Query => {
      todo!()
    },
  };



  cli_result
}
