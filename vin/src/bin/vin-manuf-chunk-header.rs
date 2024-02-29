use anyhow::Result;
use shared::io::process_stdin_send_chunk_header;
use vin::vin::vin_manuf;

fn main() -> Result<()> {
    process_stdin_send_chunk_header(vin_manuf);

    Ok(())
}
