use anyhow::Result;
use shared::io::process_stdin;
use std::boxed::Box;
use url::url::has_url;

fn main() -> Result<()> {
    process_stdin(Box::new(has_url));

    Ok(())
}
