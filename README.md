[![Build and test](https://github.com/Jacalz/sorteraw/actions/workflows/rust.yml/badge.svg)](https://github.com/Jacalz/sorteraw/actions/workflows/rust.yml)

# sorteraw
A tool to sort camera raw files into folders based on capture date.
This came about from the need to categorize protos from my camera SD card into folders based on the date they were captured.

```
Usage: sorteraw [OPTIONS] <SRC> <DST>

Arguments:
  <SRC>  Source directory to look for files to sort
  <DST>  Destination directory where folders will be created

Options:
  -m, --move-files  Move files instead of copying them
  -h, --help        Print help
  -V, --version     Print version
```
