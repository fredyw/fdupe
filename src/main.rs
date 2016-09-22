extern crate fdupe;
extern crate getopts;

use std::env;
use std::process;
use std::path::Path;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn err_and_exit(message: &str) {
    println!("Error: {}", message);
    process::exit(1);
}

fn err_req_opt(opt: &str) {
    err_and_exit(&format!("{} is a required option", opt));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("n", "num", "set number of edit distace", "NUM");
    opts.optopt("d", "dir", "set directory where to look for duplicate files", "DIRECTORY");
    opts.optopt("t", "filter", "set the file name regex filter in the directory", "FILTER");
    opts.optflag("h", "help", "print this help menu");
    if args.len() == 1 {
        print_usage(&program, opts);
        return;
    }
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            err_and_exit(&f.to_string());
            return
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let dir = match matches.opt_str("d") {
        Some(d) => {
            {
                let path = Path::new(&d);
                if !path.is_dir() {
                    err_and_exit(&format!("{} is not a directory", d));
                    return
                }
            }
            d
        }
        None => {
            err_req_opt("-d is a required option");
            return
        }
    };
    let n_ed = match matches.opt_str("n") {
        Some(n) => {
            let n = match n.parse::<i32>() {
                Ok(n) => { n }
                Err(f) => {
                    err_and_exit(&f.to_string());
                    return
                }
            };
            if n < 0 {
                err_and_exit(&format!("number of edit distance must be >= 0, got {}", n));
                return
            }
            n
        }
        None => {
            err_req_opt("-n is a required option");
            return
        }
    };
    let filter = matches.opt_str("t");
    match fdupe::fdupe::find_duplicates(&dir, n_ed, filter) {
        Ok(()) => {}
        Err(err) => {
            err_and_exit(&err);
            return
        }
    }
}
