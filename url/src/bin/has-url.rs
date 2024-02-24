use anyhow::Result;
use shared::io::process_stdin;
use url::url::has_url;

fn main() -> Result<()> {
    process_stdin(has_url);

    Ok(())
}
