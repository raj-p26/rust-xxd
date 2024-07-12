# rust-xxd

This is an implementation of UNIX command xxd in rust.

To get started visit [rustup](https://rustup.rs).

Make sure rust is installed in your local machine by checking:
```shell
rustc --version
```

After making sure that rust is succesfully installed in your local machine, clone this repo:
```shell
git clone https://github.com/raj-p26/rust-xxd
```

`cd` into the rust-xxd directory:
```shell
cd rust-xxd/
```

run the project using `cargo`:
```shell
cargo run
```

----------------

Commamd flags:
- `file_name`: The file you want to hex dump.
- `--characters | -c`: Characters/bytes you want to hex dump in one line. Default is 16.
- `--group | -g`: Number of characters to group. Default is 2.
- `--include | -i`: Print values of hex dump in C-style array elements.
- `--plain | -p`: Print hex dump in plain text i.e. No pretty print.
- `--limit | -l`: Dump hex of limited n characters. Default is 0(no limit).
- `--skip | -s`: Skip to offset n. Default is 0(No offset).
- `--binary | -b`: Pretty print binary instead of hexadecimal.

## TODO

- Add stdin when no file is given.
- Redirect output to specified target file.
- Write unit test for each flags.
