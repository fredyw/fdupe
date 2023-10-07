extern crate clap;
extern crate fdupe;

use clap::Parser;
use std::path::Path;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory where to look for duplicate files
    #[arg(short, long)]
    dir: String,

    /// File name regex filter in the directory
    #[arg(short, long)]
    filter: Option<String>,

    /// Number of edit distance
    #[arg(short, long)]
    num: u32,
}

fn err_and_exit(message: &str) {
    eprintln!("Error: {}", message);
    process::exit(1);
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.dir);
    if !path.is_dir() {
        err_and_exit(&format!("{} is not a directory", args.dir));
        return;
    }
    match fdupe::fdupe::find_duplicates(&args.dir, args.num, args.filter) {
        Err(err) => {
            err_and_exit(&err);
            return;
        }
        _ => {}
    }
}
