extern crate kuchiki;
extern crate string_cache;
extern crate hyper;
extern crate clap;
extern crate ansi_term;
extern crate rand;

use std::io::Write;
use std::str::FromStr;
use std::process::exit;

use clap::App;
use ansi_term::Colour::{Red, Yellow};
use rand::{thread_rng, sample};

mod page;
mod dictd;

fn main() {
    let matches = App::new("urdict")
        .version("0.2.0")
        .author("Ning Sun <sunng@about.me>")
        .about("Urban Dictionary from command-line")
        .args_from_usage(
            "[WORD]... 'Word to find on Urban Dictionary'
             [compact]... -c 'Output definition only'
             [sound]... -s 'Print a sound url only'
             [daemon]... -d 'Run urdict as a dictd server'
             --port=[port] 'The port to listen to, default: 2628'
             --host=[host] 'The host to listen to, default: 127.0.0.1'")
        .get_matches();

    // running as daemon
    if matches.occurrences_of("daemon") > 0 {
        let host = matches.value_of("host").unwrap_or("127.0.0.1");
        let port = u16::from_str(matches.value_of("port").unwrap_or("2628")).unwrap();

        dictd::start_server(host, port);

        return;
    }

    let def = if let Some(word) = matches.value_of("WORD"){
        page::find_on_urban_dict(&word)
    } else {
        page::find_word_of_the_day()
    };
    if let Some(def) = def {
        if matches.occurrences_of("sound") == 0 {
            let compact = matches.occurrences_of("compact") > 0;
            if !compact {
                println!("{}", Yellow.bold().paint(&def.word));
            }
            println!("{}", def.def);
            if !compact && def.example.is_some(){
                println!("");
                println!("Example:\n{}", def.example.unwrap());
            }
            if !compact {
                println!("(Author: {}, {}, Def ID: {})", def.contributor, def.date, def.id);
            }
        } else {
            if !def.sounds.is_empty() {
                let mut rng = thread_rng();
                let sample: Vec<&String> = sample(&mut rng, def.sounds.iter(), 1);
                println!("{}", sample[0]);
            } else {
                exit(128);
            }
        }
    } else {
        writeln!(&mut std::io::stderr(), "{}", Red.paint("Word not found")).unwrap();
        exit(127);
    }
}
