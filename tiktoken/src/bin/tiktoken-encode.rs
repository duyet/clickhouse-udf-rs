use anyhow::Result;
use shared::io::process_stdin;
use tiktoken::tiktoken::tiktoken_encode;

fn main() -> Result<()> {
    process_stdin(Box::new(tiktoken_encode));

    Ok(())
}
