use anyhow::Result;
use shared::io::process_stdin;
use url::url::extract_url;

fn main() -> Result<()> {
    process_stdin(extract_url);

    Ok(())
}
