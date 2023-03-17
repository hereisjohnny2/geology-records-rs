use geology_records_rs::cli;
use std::io;

fn main() -> io::Result<()> {
    println!("Geology Records - v0.1.0");

    cli::run()?;

    Ok(())
}
