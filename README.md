# fdupe
A tool written in Rust to find duplicate files based on file name.

### How to Build
```
cargo build
```

### Usage
```
Usage: fdupe [OPTIONS] --dir <DIR> --num <NUM>

Options:
  -d, --dir <DIR>        Directory where to look for duplicate files
  -f, --filter <FILTER>  File name regex filter in the directory
  -n, --num <NUM>        Number of edit distance
  -h, --help             Print help
  -V, --version          Print version
```    
