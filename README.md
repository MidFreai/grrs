# BTgrrs

BLOATED grrs implementation to verify patterns in files

# Usage
```
grrs <PATTERN> <FILE> [COMMAND]

Arguments:
    <PATTERN> Pattern for search
    <FILE> Path os the file

Commands: 
    table: Print in table format
    tableall: Print all lines in table format
    reverse: Print all lines that do not match the pattern
```

# Install

If you have cargo download the files 
go to project directory and run on terminal
```bash
cargo run --release
sudo cp target/release/grrs /usr/local/bin
sudo chmod +x /usr/local/bin/grrs
grrs -V
```
If not have cargo you can download the pre-compiled version copy in /usr/local/bin and do the chmod command above
