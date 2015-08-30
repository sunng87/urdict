extern crate kuchiki;
extern crate hyper;
extern crate clap;
extern crate ansi_term;

use clap::App;
use ansi_term::Colour::{Red, Yellow};

mod page;

fn main() {
    let matches = App::new("urdict")
        .version("0.1")
        .author("Ning Sun <sunng@about.me>")
        .about("Urban Dictionary from command-line")
        .args_from_usage(
            "<WORD> 'Word to find on Urban Dictionary'")
        .get_matches();
    let word = matches.value_of("WORD").unwrap();
    if let Some(def) = page::find_on_urban_dict(&word) {
        println!("{}: {}", Yellow.bold().paint(&def.word), def.def)
    } else {
        println!("{}", Red.paint("Word not found"));
    }
}
