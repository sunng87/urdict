extern crate kuchiki;
extern crate hyper;
extern crate clap;

mod page;

fn main() {
    println!("Hello, world!");
    if let Some(def) = page::find_on_urban_dict("LGTSS") {
        println!("{}: {}", def.word, def.def)
    };
}
