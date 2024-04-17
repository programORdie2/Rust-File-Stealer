# Rust File Stealer
## Collect ("steal") and zip files in just seconds

Easy find all the files on a computer and put them in a ZIP file with this program.

## How to install?
### Clone this repo
```
git clone https://github.com/programORdie2/Rust-File-Stealer.git
cd  Rust-File-Stealer
```
### Run the file
```
cargo build
cargo run
```
That's it!

## Options
### Compression type
Sets the compression type (0 none [fastest, biggest file size], 1 deflated [slower, smaller], 2cbzip [slowest, smallest file size]), default is 1.
<br>
Example:
```
cargo run -- --compression 0
```

### Max scan size
Defines the max file size the program should copy in MB, default is 5.
<br>
Example:
```
cargo run -- --max_size 2
```

### Scan USB drives or other stations too
If the program should scan all mounted drives, use this flag, default is false.
<br>
Example:
```
cargo run -- --drives
```

## Notes
 - Currently, this project only works on windows.
 - I forgot to test the drives function.
<br><br>
**That's it, I hope you like it! Feel free to [contact me](https://pod.stio.studio/#contact) if you find any bugs.**
