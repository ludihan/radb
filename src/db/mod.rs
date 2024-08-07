use std::path::PathBuf;

use clap::Parser;

mod parser;
mod lexer;
mod connection;
mod db_writer;

/// Relational algebra database management system
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct DatabaseArgs {
    /// Database file
    file: Option<PathBuf>,
}

pub struct Database {
    args: DatabaseArgs
}

impl Database {
    pub fn new () -> Database {
        Database {
            args: DatabaseArgs::parse(),
        }
    }

    pub fn greetings(&self) {
        println!(
            "{} version {}\nEnter \".help\" for usage hints.",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        );

        if self.args.file.is_none() {
            println!("Connected to a transient in-memory database.\nUse \".open FILENAME\" to reopen on a persistent database.");
        }
    }

    pub fn process_line(&self, line: &str) {
        /*
        match &self.args.file {
            Some(file) => operate_db_file(file),
            None => println!(),
        }
        */
    }
}
