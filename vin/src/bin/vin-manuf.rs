use anyhow::Result;
use shared::io::process_stdin;
use vin::vin::vin_manuf;

fn main() -> Result<()> {
    process_stdin(vin_manuf);

    Ok(())
}
