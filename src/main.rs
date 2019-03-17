use std::process;
use structopt::StructOpt;

use q_boilerplate::{gen_viz, Error as CrateError};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "q_boilerplate",
    about = "Generates boilerplate code for Qlik Extensions"
)]
enum Commands {
    #[structopt(name = "mashup", about = "Creates mashup extension template")]
    Mashup(Options),
    #[structopt(
        name = "visualisation",
        about = "Creates Visualisation extension template"
    )]
    Visualisation(Options),
}

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(help = "Project name")]
    name: String,
    #[structopt(
        help = "Opt out of creating a git repo for the new extension",
        long = "no-git"
    )]
    no_git: bool,
}

fn main() -> Result<(), CrateError> {
    let opt = Commands::from_args();

    match &opt {
        Commands::Mashup(_) => {
            eprintln!("Mashup Template not yet implemented ");
            process::exit(1);
        }
        Commands::Visualisation(options) => {
            gen_viz(&options.name, !options.no_git)?;
        }
    };

    Ok(())
}
