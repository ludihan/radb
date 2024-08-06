use std::path::Path;

use crate::Args;

pub fn process_cmd(args: &Args, line: &str) {
    match &args.file {
        Some(file) => operate_db_file(file),
        None => println!(),
    }
}

fn operate_db_file(path: &Path) {}
