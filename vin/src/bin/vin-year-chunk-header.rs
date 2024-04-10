use anyhow::Result;
use shared::io::process_stdin;
use std::boxed::Box;
use vin::vin::vin_year;

fn main() -> Result<()> {
    process_stdin(Box::new(vin_year));

    Ok(())
}
