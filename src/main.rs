use std::path::PathBuf;
use structopt::StructOpt;

use q_boilerplate::error::Error;

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
        help = "Optional output path, otherwise the current working directory",
        parse(from_os_str)
    )]
    path: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let opt = Commands::from_args();

    match &opt {
        Commands::Mashup(options) => {}
        Commands::Visualisation(options) => {}
    }

    println!("{:?}", opt);

    Ok(())
}
