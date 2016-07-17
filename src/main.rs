extern crate getopts;
use getopts::Options;
use std::env;
use std::path::Path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("n", "", "set number of edit distace", "EDIT_DISTANCE");
    opts.optopt("d", "", "set directory where to look for duplicate files", "DIRECTORY");
    opts.optflag("h", "help", "print this help menu");
    if args.len() == 1 {
        print_usage(&program, opts);
        return;
    }
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
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
                    panic!("{} is not a directory", d)
                }
            }
            d
        }
        None => { panic!("-d is a required option") }
    };
    let n_ed = match matches.opt_str("n") {
        Some(n) => {
            let n = match n.parse::<i32>() {
                Ok(n) => { n }
                Err(f) => { panic!(f.to_string()) }
            };
            if n < 0 {
                panic!("number of edit distance must be >= 0, got {}", n);
            }
            n
        }
        None => { panic!("-n is a required option") }
    };
    // TODO
}
