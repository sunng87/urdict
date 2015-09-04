extern crate kuchiki;
extern crate string_cache;
extern crate hyper;
extern crate clap;
extern crate ansi_term;
extern crate rand;

use clap::App;
use ansi_term::Colour::{Red, Yellow};
use rand::{thread_rng, sample};

mod page;

fn main() {
    let matches = App::new("urdict")
        .version("0.1")
        .author("Ning Sun <sunng@about.me>")
        .about("Urban Dictionary from command-line")
        .args_from_usage(
            "[WORD]... 'Word to find on Urban Dictionary'
             [example]... -e 'Show example if any'
             [sound]... -s 'Print a sound url only'")
        .get_matches();
    let def = if let Some(word) = matches.value_of("WORD"){
        page::find_on_urban_dict(&word)
    } else {
        page::find_word_of_the_day()
    };
    if let Some(def) = def {
        if matches.occurrences_of("sound") == 0 {
            println!("{}\n{}", Yellow.bold().paint(&def.word), def.def);
            println!("");
            if matches.occurrences_of("example") > 0
                && def.example.is_some(){
                    println!("Example: {}", def.example.unwrap());
                }
            println!("(Author: {}, {}, Def ID: {})", def.contributor, def.date, def.id);
        } else {
            if !def.sounds.is_empty() {
                let mut rng = thread_rng();
                let sample: Vec<&String> = sample(&mut rng, def.sounds.iter(), 1);
                println!("{}", sample[0]);
            } else {
            }
        }
    } else {
        println!("{}", Red.paint("Word not found"));
    }
}
