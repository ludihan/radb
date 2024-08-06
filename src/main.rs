use std::fs;
use std::path::PathBuf;

use clap::Parser;
use home::home_dir;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod db;

/// Relational algebra database management system
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Database file
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut rl = DefaultEditor::new()?;

    let mut home: PathBuf;
    match home_dir() {
        Some(dir) => home = dir,
        None => home = PathBuf::new(),
    }
    home.push(r".radb_history");
    if !home.exists() {
        let _ = fs::write(&home, "");
    }

    let history_file = home.as_path();

    /*
    println!("DB File: {:?}", args.file);
    println!("History File: {:?}", history_file);
    */

    let _ = rl.load_history(&history_file);
    /*
    if history.is_err() {
        println!("{:?}", history)
    }
    */

    println!(
        "{} version {}\nEnter \".help\" for usage hints.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    if args.file.is_none() {
        println!("Connected to a transient in-memory database.\nUse \".open FILENAME\" to reopen on a persistent database.");
    }

    loop {
        let readline = rl.readline(&format!("{}> ", env!("CARGO_PKG_NAME")).to_string());
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                db::process_cmd(&args, &line);
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Program interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("bye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    let _ = rl.save_history(&history_file);
    Ok(())
}
