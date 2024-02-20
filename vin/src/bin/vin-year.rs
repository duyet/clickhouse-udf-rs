use anyhow::Result;
use shared::io::process_stdin;
use vin::vin::vin_year;

fn main() -> Result<()> {
    process_stdin(vin_year);

    Ok(())
}
