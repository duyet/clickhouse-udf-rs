use std::boxed::Box;
use anyhow::Result;
use shared::io::process_stdin;
use vin::vin::vin_cleaner;

fn main() -> Result<()> {
    process_stdin(Box::new(vin_cleaner));

    Ok(())
}
