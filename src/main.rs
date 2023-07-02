use std::{
    fs::read_dir,
    io::{self, Error},
};

fn main() {
    let files = collect_files_from_filepath("./writing");

    println!("Files {:?}", files);
}

fn collect_files_from_filepath(file_path: &str) -> Result<Vec<std::path::PathBuf>, Error> {
    let entries: Vec<std::path::PathBuf> = read_dir(file_path)?
        .map(|file| file.map(|result| result.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}
