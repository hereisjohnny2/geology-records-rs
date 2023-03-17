pub mod io_utils;
pub mod samples_database;

use std::io;

use crate::cli::{
    io_utils::{create_rock_sample_entry, safe_read_and_parse},
    samples_database::SamplesDatabase,
};

fn render_menu() -> usize {
    println!("\nSelect Option:");
    println!("1 - Add Record");
    println!("2 - List Records");
    println!("3 - Remove Record");
    println!("4 - Sair Record");

    safe_read_and_parse::<usize>("Choice")
}

fn run_option(db: &mut SamplesDatabase) {
    let choice = render_menu();

    match choice {
        1 => {
            let sample = create_rock_sample_entry(db.count());
            db.add(sample);

            run_option(db)
        }
        2 => {
            println!("{}", db.to_string());
            run_option(db)
        }
        3 => {
            todo!("remove sample");
        }
        4 => {
            println!("exit...");
        }
        _ => {
            println!("Invalid Option");
            run_option(db)
        }
    }
}

pub fn run() -> io::Result<()> {
    let mut samples_db = SamplesDatabase::new();
    run_option(&mut samples_db);

    // println!("{}", "-".repeat(30));
    //
    // let mut samples_file = File::create("samples.txt")?;
    // samples_file.write_all(samples_db.to_string().as_bytes())?;
    //
    Ok(())
}
