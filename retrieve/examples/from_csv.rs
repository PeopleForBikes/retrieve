//! Reads a city ratings CSV file and display the first record in full and short
//! code version.
//!
//! Run with:
//! ```not_rust
//! cargo run -q --example from_csv
//! ```

use color_eyre::{eyre::Report, Result};
use pfbcore::scorecard::{ScoreCard, ShortScoreCard};
use retrieve::setup;

const CITY_RATINGS_CSV: &'static str = "examples/city_ratings_2021_v15.csv";

fn main() -> Result<(), Report> {
    // Setup the application.
    setup()?;

    let scorecards = ScoreCard::from_csv(CITY_RATINGS_CSV)?;
    let sc = scorecards.first().unwrap();
    dbg!(&sc);
    let short_sc = ShortScoreCard::from(sc);
    dbg!(&short_sc);

    Ok(())
}
