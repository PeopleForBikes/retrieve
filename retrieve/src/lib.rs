use color_eyre::{eyre::Report, Result};

pub mod cli;

/// Setup the application.
///
/// Set up the `color_eyre` hooks.
pub fn setup() -> Result<(), Report> {
    color_eyre::install()?;

    Ok(())
}
