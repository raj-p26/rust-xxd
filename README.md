# rust-xxd

This is an implementation of UNIX command xxd in rust.

To get started visit [rustup](https://rustup.rs).

Make sure rust is installed in your local machine by checking:
```shell
$ rustc --version
```

After making sure that rust is succesfully installed in your local machine, clone this repo:
```shell
$ git clone https://github.com/raj-p26/rust-xxd
```

`cd` into the rust-xxd directory:
```shell
$ cd rust-xxd/
```

run the project using `cargo`:
```shell
$ cargo run
```

----------------

Command flags:
- `file_name`: The file you want to hex dump.
- `--binary | -b`: Pretty print binary instead of hexadecimal.
- `--characters | -c`: Characters/bytes you want to hex dump in one line. Default is 16.
- `--decimal | -d`: Print offset in decimal. Default is hexadecimal.
- `--group | -g`: Number of characters to group. Default is 2.
- `--include | -i`: Print values of hex dump in C-style array elements.
- `--limit | -l`: Dump hex of limited n characters. Default is 0(no limit).
- `--plain | -p`: Dump hex in plain format.
- `--skip | -s`: Skip to offset n. Default is 0(No offset).
- `--uppercase | -u`: Dump hex in uppercase format.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
