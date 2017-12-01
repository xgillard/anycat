extern crate anycat;

#[macro_use]
extern crate clap;

use std::io::prelude::*;

fn main() {
    let app = clap::App::new("anycat")
                .version(crate_version!())
                .author (crate_authors!())
                .about  (crate_description!())
                .arg(clap::Arg::with_name("file")
                        .required(true)
                        .value_name("FILE_NAME"));

    let args = app.get_matches();

    let x = anycat::readfile(args.value_of("file").unwrap());

    for l in x.lines() {
        println!("{}", l.unwrap());
    }
}
