use clap::{ArgEnum, Parser, ValueHint};
use pfbcore::Dataset;
use std::convert::From;
use std::path::PathBuf;

/// Describe all the available city datasets.
///
/// This enum must be in sync with [`pfb-core::Dataset`].
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, ArgEnum)]
pub enum CliDataset {
    NeighborhoodWays,
    NeighborhoodOverallScores,
}

impl From<Dataset> for CliDataset {
    fn from(dataset: Dataset) -> Self {
        match dataset {
            Dataset::NeighborhoodOverallScores => CliDataset::NeighborhoodOverallScores,
            Dataset::NeighborhoodWays => CliDataset::NeighborhoodWays,
        }
    }
}

impl From<CliDataset> for Dataset {
    fn from(dataset: CliDataset) -> Self {
        match dataset {
            CliDataset::NeighborhoodOverallScores => Dataset::NeighborhoodOverallScores,
            CliDataset::NeighborhoodWays => Dataset::NeighborhoodWays,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Specify a CSV file containing the list of city datasets to download
    #[clap(long)]
    pub from_csv: Option<String>,

    /// Specify the number of files to download simultaneously
    #[clap(short, long, default_value_t = 25)]
    pub parallel_requests: u16,

    /// Specify the number times to retry a failing download
    #[clap(short, long, default_value_t = 3)]
    pub retries: u16,

    /// Specify the destination directory
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath, default_value = "output")]
    pub destination_folder: PathBuf,

    /// Specify the dataset to retrieve
    #[clap(arg_enum)]
    pub dataset: CliDataset,
}
