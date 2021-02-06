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

    let fname= args.value_of("file").unwrap();
    
    match anycat::readfile(fname) {
        Err(e)=> {println!("Could not open {}. \nReason: {:?}", fname, e);}
        Ok(x) => {
                for l in x.lines() {
                println!("{}", l.unwrap());
            }
        }
    }
}
