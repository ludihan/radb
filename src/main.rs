use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Database file
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut rl = DefaultEditor::new()?;

    let history = rl.load_history("~/radb_history.txt");
    if history.is_err() {
        println!("{:?}", history);
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt");
    Ok(())
}
