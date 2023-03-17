pub mod io_utils;
pub mod samples_database;

use std::{
    fs::File,
    io::{self, Write},
};

use crate::cli::{
    io_utils::{create_rock_sample_entry, safe_read},
    samples_database::SamplesDatabase,
};

pub fn run() -> io::Result<()> {
    let mut samples_db = SamplesDatabase::new();

    loop {
        println!("{}", "-".repeat(30));
        let rock_sample = create_rock_sample_entry();
        samples_db.add(rock_sample);

        let choice = safe_read("Add another sample? (Y/N)");
        if choice.ne("y") || choice.ne("Y") {
            break;
        }
    }

    println!("{}", "-".repeat(30));

    let mut samples_file = File::create("samples.txt")?;
    samples_file.write_all(samples_db.to_string().as_bytes())?;

    Ok(())
}
