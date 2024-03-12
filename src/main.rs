mod utils;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use minecraft_formats::formats::{printer::Print, statue::Statue};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: PathBuf,
    output: PathBuf,
}

enum FileType {
    Statue(Statue),
    Print(Print),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let ext_input = cli
        .input
        .extension()
        .context("Expected extension on input file")?
        .to_str()
        .unwrap();
    let ext_output = cli
        .output
        .extension()
        .context("Expected extension on output file")?
        .to_str()
        .unwrap();

    let input = utils::to_filetype(ext_input.into(), &cli.input)?;

    match ext_output {
        "3dj" => {
            let print: Print = match input {
                FileType::Statue(statue) => statue.into(),
                FileType::Print(print) => print, // Do nothing if its the same thing
            };

            let json = serde_json::to_string_pretty::<Print>(&print)
                .context("Failed to serialize converted format to string")?;

            utils::write_to_file(json, cli.output)?;
        }
        "statue" => {
            let statue: Statue = match input {
                FileType::Statue(statue) => statue, // Do nothing if its the same thing
                FileType::Print(print) => print.into(),
            };
            let json = serde_json::to_string_pretty::<Statue>(&statue)
                .context("Failed to serialize converted format to string")?;

            utils::write_to_file(json, cli.output)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
