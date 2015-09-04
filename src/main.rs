extern crate kuchiki;
extern crate string_cache;
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
            "<WORD> 'Word to find on Urban Dictionary'
             [example]... -e 'Show example if any'")
        .get_matches();
    let word = matches.value_of("WORD").unwrap();
    if let Some(def) = page::find_on_urban_dict(&word) {
        println!("{}\n{}", Yellow.bold().paint(&def.word), def.def);
        println!("");
        if matches.occurrences_of("example") > 0
            && def.example.is_some(){
                println!("Example: {}", def.example.unwrap());
            }
        println!("({} on {}, Def ID: {})", def.contributor, def.date, def.id);
    } else {
        println!("{}", Red.paint("Word not found"));
    }
}
