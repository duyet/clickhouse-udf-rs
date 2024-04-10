use anyhow::Result;
use shared::io::process_stdin;
use std::boxed::Box;

use parse_wkt::parse_wkt::parse_wkt;

fn main() -> Result<()> {
    process_stdin(Box::new(parse_wkt));

    Ok(())
}
