use anyhow::Result;
use shared::io::process_stdin;
use vin::vin::vin_cleaner;

fn main() -> Result<()> {
    process_stdin(vin_cleaner);

    Ok(())
}
