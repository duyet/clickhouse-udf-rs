use anyhow::Result;
use shared::io::process_stdin;
use std::boxed::Box;
use vin::vin::vin_cleaner;

fn main() -> Result<()> {
    process_stdin(Box::new(vin_cleaner));

    Ok(())
}
