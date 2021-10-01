use clap::{App, Arg};
use std::path::{Path, PathBuf};
use std::fs;
use libshaiya::archive::Archive;
use libshaiya::archive::file::SFolder;

/// The version of this tool.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The entry point for the archive extractor. This is designed to efficiently extract the contents
/// of Shaiya archive files.
fn main() -> anyhow::Result<()> {
    let args = App::new("Shaiya Archive Extractor")
        .version(VERSION)
        .author("Triston Plummer (Cups)")
        .arg(Arg::with_name("header")
            .short("h")
            .long("header")
            .value_name("FILE")
            .help("The path to the header file.")
            .default_value("data.sah")
            .takes_value(true))
        .arg(Arg::with_name("datafile")
            .short("d")
            .long("data")
            .value_name("FILE")
            .help("The path to the data file.")
            .default_value("data.saf")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("The path to the output directory.")
            .default_value("extracted")
            .takes_value(true))
        .get_matches();

    // Get the paths to the required files.
    let header_file = Path::new(args.value_of("header").unwrap());
    let data_file = Path::new(args.value_of("datafile").unwrap());
    let output = Path::new(args.value_of("output").unwrap());
    println!("Parsing archive with header: {:?} and datafile: {:?}.", header_file, data_file);

    // Parse the archive.
    let mut archive = Archive::open(header_file, data_file)?;
    println!("Successfully parsed archive.");

    // Set up the folder and files to extract from.
    let folder = archive.root.clone();
    let root_path = output.join(Path::new(&folder.name));

    // Extract the archive.
    extract(&mut archive, &root_path, folder)?;
    println!("Finished extracting.");
    Ok(())
}

/// Extracts a folder.
///
/// # Arguments
/// * `archive` - The archive to extract from.
/// * `path`    - The path to extract to.
/// * `folder`  - The folder to extract.
fn extract(archive: &mut Archive, path: &Path, folder: SFolder) -> anyhow::Result<()> {
    // Create the path if it doesn't exist.
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let mut display_path = PathBuf::new();
    path.components().skip(1)
        .for_each(|c| display_path.push(c));
    println!("Extracting folder: {:?}", &display_path);

    for file in folder.files() {
        let data = archive.file_data(file)?;
        fs::write(path.join(Path::new(&file.name)), data)?;
    }

    for folder in folder.subdirectories() {
        let subdir_path = path.join(Path::new(&folder.name));
        extract(archive, &subdir_path, folder.clone())?;
    }
    Ok(())
}