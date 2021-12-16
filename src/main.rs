extern crate clap;

use ipynb::Notebook;
use std::fs::read_to_string;
use std::path::Path;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Ipynb checker")
        .version("0.0.1")
        .author("Matthias Bussonnier <bussonniermatthias@gmail.com>")
        .about("try to parse ipynb with rust")
        .arg(
            Arg::with_name("FILES")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let file_names: Vec<_> = matches.values_of("FILES").unwrap().collect();

    for file_name in file_names {
        println!("... {}", file_name);
        let path = Path::new(&file_name);

        let input = read_to_string(path).unwrap();

        let maybe_notebook: Result<Notebook, _> = serde_json::from_str(&input);
        maybe_notebook.unwrap();
        //if let Ok(_n) = maybe_notebook {
        //    println!("ok {}", file_name);
        //} else {
        //    println!("failed {}", file_name);
        //}
    }
}
