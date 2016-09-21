# fdupe
A tool written in Rust to find duplicate files based on file name.

### How to Build
```
cargo build
```

### Usage
```
Usage: ./fdupe [options]

Options:
    -n EDIT_DISTANCE    set number of edit distace
    -d DIRECTORY        set directory where to look for duplicate files
    -t FILTER           set the file name regex filter
    -h, --help          print this help menu
```    