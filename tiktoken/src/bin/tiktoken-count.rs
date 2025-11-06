use anyhow::Result;
use shared::io::process_stdin;
use std::boxed::Box;
use tiktoken::tiktoken::tiktoken_count;

fn main() -> Result<()> {
    process_stdin(Box::new(tiktoken_count));

    Ok(())
}
