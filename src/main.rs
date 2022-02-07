use clap::Parser;
use color_eyre::{eyre::Report, Result};
use downloader::{Download, Downloader};
use retrieve::cli::Args;
use retrieve::{setup, City};
use std::fs;

fn main() -> Result<(), Report> {
    // Setup the application.
    setup()?;

    // Read the CLI arguments.
    let args = Args::parse();

    // Prepare the variable holding the list of cities to process.
    let mut cities: Vec<City> = Vec::new();

    // Prepare the list of items to retrieve from a CSV file.
    if let Some(csv) = args.from_csv {
        cities = City::from_csv(csv)?;
    }

    // Ensure the output folder exists.
    if !args.destination_folder.exists() {
        fs::create_dir_all(&args.destination_folder)?;
    }

    // Prepare the downloader.
    let mut downloader = Downloader::builder()
        .download_folder(&args.destination_folder)
        .parallel_requests(args.parallel_requests)
        .retries(args.retries)
        .build()
        .unwrap();

    // Prepare the downloads.
    let downloads = cities
        .iter()
        .filter(|c| !c.uuid.is_empty())
        .map(|c| {
            downloader::Download::new(c.url(args.dataset).unwrap().as_str()).file_name(
                std::path::Path::new(&format!("{}.{}", &c.full_name(), &args.dataset.extension())),
            )
        })
        .collect::<Vec<Download>>();

    // Start the download operations.
    let _dl_result = downloader.download(&downloads);
    // dbg!(&dl_result);

    Ok(())
}
