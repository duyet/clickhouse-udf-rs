use anyhow::Result;
use shared::io::process_stdin_send_chunk_header;
use std::boxed::Box;
use vin::vin::vin_cleaner;

fn main() -> Result<()> {
    process_stdin_send_chunk_header(Box::new(vin_cleaner));

    Ok(())
}
