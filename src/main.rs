use clap::Parser;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "unzip-rs")]
#[command(about = "A fast zip file extractor written in Rust")]
struct Args {
    #[arg(help = "Input zip file")]
    input: PathBuf,

    #[arg(help = "Destination folder")]
    destination: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let started = std::time::Instant::now();
    // Create destination directory if it doesn't exist
    fs::create_dir_all(&args.destination)?;

    // Open the zip file
    let file = File::open(&args.input)?;
    // let buf_reader = std::io::BufReader::with_capacity(8 * 1024 * 1024, file);
    println!("Opening zip file: {:?}", &args.input);

    let mut archive =
        zip::ZipArchive::new(file).expect("Failed to open zip file for reading");

    println!(
        "Extracting zip file: {:?} to {:?}",
        &args.input, &args.destination
    );

    archive.extract(&args.destination)?;
    println!("Time taken: {:?}", started.elapsed());

    Ok(())
}
