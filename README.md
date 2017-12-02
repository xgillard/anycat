# Anycat
Anycat is an extremely simple tool that lets you unpack some single file archive
and print its content to the standard output.

It supports quite a few decompression algorithms. Namely:

- GZIP (.gz, .gzip)
- BZIP2 (.bz2, .bzip)
- LZMA (.xz, .lzma)
- BROTLI (.br , .brotli)
- ZLIB (.z , .zlib)
- DEFLATE (.dfl)

Its indended use is to simplify the cases where you want to chose between cat,
gzcat, bzcat, ... to print a file. Whenever you want to do this, you might
just as well call `anycat <your_file>` and you should get the right result.


## Tech note:
This project is written in plain Rust, so you should be able to build it w/o
any trouble by simply issuing the `cargo build --release` command.

## Credits
Even though I wrote `anycat`, the heavy lifting was done by the guys who wrote
the many decompression crates for Rust and made them available on crates.io.
If you are one of these guys, thank you !
