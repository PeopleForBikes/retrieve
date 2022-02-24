use color_eyre::{eyre::Report, Result};
use retrieve::{setup, City, ScoreCard};
// use std::path::Path;

const CITY_RATINGS_CSV: &'static str = "examples/city_ratings_2021_v15.csv";

fn main() -> Result<(), Report> {
    // Setup the application.
    setup()?;

    // let cities = City::from_csv(CITY_RATINGS_CSV)?;
    // dbg!(&cities);
    let scorecards = ScoreCard::from_csv(CITY_RATINGS_CSV)?;
    dbg!(&scorecards);

    Ok(())
}
