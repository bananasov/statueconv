use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use shared::formats::{printer::Print, statue::Statue};

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

    let input = to_filetype(ext_input.into(), &cli.input)?;

    match ext_output {
        "3dj" => {
            let print: Print = match input {
                FileType::Statue(statue) => statue.into(),
                FileType::Print(print) => print, // Do nothing if its the same thing
            };

            let json = serde_json::to_string_pretty::<Print>(&print)
                .context("Failed to serialize converted format to string")?;

            let mut file = File::options()
                .write(true)
                .create(true)
                .open(cli.output)
                .context("Failed to open output file for writing")?;
            file.write_all(json.as_bytes())
                .context("Failed to write serialized statue to output file")?;
        }
        "statue" => {
            let statue: Statue = match input {
                FileType::Statue(statue) => statue, // Do nothing if its the same thing
                FileType::Print(print) => print.into(),
            };
            let json = serde_json::to_string_pretty::<Statue>(&statue)
                .context("Failed to serialize converted format to string")?;

            let mut file = File::options()
                .write(true)
                .create(true)
                .open(cli.output)
                .context("Failed to open output file for writing")?;
            file.write_all(json.as_bytes())
                .context("Failed to write serialized statue to output file")?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn read_to_string(path: PathBuf) -> anyhow::Result<String> {
    let mut file = File::open(path).context("Failed to open file")?;
    let mut contents = String::new();
    let _ = file
        .read_to_string(&mut contents)
        .context("Failed to read file contents")?;

    Ok(contents)
}

fn to_filetype(extension: String, path: &PathBuf) -> anyhow::Result<FileType> {
    let extension = extension.as_str();
    match extension {
        "3dj" => {
            let contents = read_to_string(path.to_path_buf())?;

            let print: Print = serde_json::from_str(&contents)
                .context("Failed to deserialize contents of input file to a 3D Print")?;
            Ok(FileType::Print(print))
        }
        "statue" => {
            let contents = read_to_string(path.to_path_buf())?;

            let statue: Statue = serde_json::from_str(&contents)
                .context("Failed to deserialize contents of input file to a Statue")?;
            Ok(FileType::Statue(statue))
        }
        _ => unreachable!(),
    }
}
