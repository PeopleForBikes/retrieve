use clap::Parser;
use color_eyre::{eyre::Report, Result};
use downloader::{Download, Downloader};
use retrieve::cli::Args;
use retrieve::{setup, City};

fn main() -> Result<(), Report> {
    // Setup the application.
    setup()?;

    // Read the CLI arguments.
    let args = Args::parse();
    // dbg!(&args);

    // Prepare the variable holding the list of cities to process.
    let mut cities: Vec<City> = Vec::new();

    // Prepare the list of items to retrieve from a CSV file.
    if let Some(csv) = args.from_csv {
        cities = City::from_csv(csv)?;
        // dbg!(&cities);
    }

    // Prepare the downloader.
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new(&args.destination_folder))
        .parallel_requests(args.parallel_requests)
        .retries(args.retries)
        .build()
        .unwrap();

    // Prepare the downloads.
    let downloads = cities
        .iter()
        .filter(|c| !c.uuid.is_empty())
        // .take(5)
        .map(|c| {
            downloader::Download::new(&c.url(args.dataset).unwrap().as_str())
                .file_name(std::path::Path::new(&c.zip_name()))
        })
        .collect::<Vec<Download>>();
    // dbg!(&downloads.len());

    // Start the download operations.
    let _dl_result = downloader.download(&downloads);
    // dbg!(&dl_result);

    Ok(())
}
