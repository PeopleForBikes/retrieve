use crate::Dataset;
use clap::{Parser, ValueHint};
use std::path::PathBuf;

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
    pub dataset: Dataset,
}
