extern crate flate2;
extern crate bzip2;
extern crate xz2;
extern crate brotli2;

use std::fs::File;
use std::io::{BufRead, BufReader};

use flate2::read::*;
use bzip2::read::*;
use xz2::read::*;
use brotli2::read::*;

/// Returns a BufRead to the file denoted by the given fname. The stream is possibly decompressed
/// using the following scheme. Depending on the **file extension**, the following algorithm is
/// used:
/// - .gz, .gzip  ===> GZIP
/// - .z , .zlib  ===> ZLIB
/// - .dfl        ===> DEFLATE
/// - .bz2, .bzip ===> BZ2
/// - .xz2, .lzma ===> LZMA (xz2)
/// - .br , .brotli => BROTLI
///
/// Otherwise, the file is assumed to contain plaintext and is treated as such
pub fn readfile(fname: &str) -> Box<BufRead> {
    let file = File::open(fname).expect(&format!("Could not open {}", fname));

    let canonical = fname.to_lowercase();

    // I'm sure about these ones
    if canonical.ends_with(".gz") || canonical.ends_with(".gzip") {
        return Box::new(BufReader::new(GzDecoder::new(file)));
    }
    if canonical.ends_with(".bz2") || canonical.ends_with(".bzip") {
        return Box::new(BufReader::new(BzDecoder::new(file)));
    }
    if canonical.ends_with(".xz") || canonical.ends_with(".lzma") {
        return Box::new(BufReader::new(XzDecoder::new(file)));
    }

    // I *guess* about these ones...
    if canonical.ends_with(".zlib") || canonical.ends_with(".z") {
        return Box::new(BufReader::new(ZlibDecoder::new(file)));
    }
    if canonical.ends_with(".dfl") {
        return Box::new(BufReader::new(DeflateDecoder::new(file)));
    }
    if canonical.ends_with(".brotli") || canonical.ends_with(".br") {
        return Box::new(BufReader::new(BrotliDecoder::new(file)));
    }

    return Box::new(BufReader::new(file));
}
