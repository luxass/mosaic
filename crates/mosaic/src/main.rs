use std::mem;

use anyhow::Result;
use clap::{arg, builder::NonEmptyStringValueParser, ArgAction, ArgGroup, ArgMatches, Command, CommandFactory, Parser, Subcommand, ValueEnum};


fn cli() -> Command {
    Command::new("mosaic")
        .about("An API for your projects")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Run the mosaic server")
        )
        .subcommand(
            Command::new("validate")
                .about("Validate a Mosaic Project Config")
                .arg(arg!(project_name: [PROJECT_NAME]))
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            todo!()
        },
        Some(("validate", sub_matches)) => {

            let mut project_name = sub_matches.get_one::<String>("project_name").map(|s| s.as_str());

            if project_name.is_none() {
                let is_mosaic_project = mosaic_utils::projects::is_mosaic_project().await;

                if !is_mosaic_project {
                    unreachable!("Could not find mosaic project.")
                }

                project_name = Some("asdas");
            }

            println!("Project Name: {}", project_name.unwrap());

            todo!("asdj")
        },
        _ => unreachable!()
    }
}
