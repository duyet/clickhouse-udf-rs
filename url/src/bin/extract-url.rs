use anyhow::Result;
use shared::io::process_stdin;
use std::boxed::Box;
use url::url::extract_url;

fn main() -> Result<()> {
    process_stdin(Box::new(extract_url));

    Ok(())
}
