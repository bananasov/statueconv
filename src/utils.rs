use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use minecraft_formats::formats::{printer::Print, statue::Statue};

use crate::FileType;

pub fn read_to_string(path: PathBuf) -> anyhow::Result<String> {
    let mut file = File::open(path).context("Failed to open file")?;
    let mut contents = String::new();
    let _ = file
        .read_to_string(&mut contents)
        .context("Failed to read file contents")?;

    Ok(contents)
}

pub fn write_to_file(contents: String, path: PathBuf) -> anyhow::Result<()> {
    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .context("Failed to open output file for writing")?;

    file.write_all(contents.as_bytes())
        .context("Failed to write serialized statue to output file")?;

    Ok(())
}

pub fn to_filetype(extension: String, path: &Path) -> anyhow::Result<FileType> {
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
