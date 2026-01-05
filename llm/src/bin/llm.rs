use anyhow::Result;
use shared::io::process_stdin;

fn main() -> Result<()> {
    process_stdin(Box::new(llm::llm));
    Ok(())
}
