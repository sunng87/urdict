use std::io::Read;

use reqwest::Client;

use select::document::Document;
use select::predicate::*;

#[derive(Debug)]
pub struct DictDef {
    pub word: String,
    pub def: String,
    pub example: Option<String>,
    pub upvote: i32,
    pub downvote: i32,
    pub contributor: String,
    pub date: String,
    pub id: String,
    pub sounds: Option<Vec<String>>,
    pub similars: Option<Vec<String>>
}

impl DictDef {
}

fn urban_dict_url (word: &str) -> String {
    let mut s = String::new();
    s.push_str("https://www.urbandictionary.com/define.php?term=");
    s.push_str(word);
    s
}

fn json_list_to_strings(json: &str) -> Vec<String> {
    json[1..json.len()-1].split(", ").map(|s| s[1..s.len()-1].to_owned()).collect()
}

fn get_def_from_doc (doc: &Document) -> Option<DictDef> {
    if let Some(panel) = doc.find(Class("def-panel")).into_selection().first() {
        if let Some(word) = panel.find(Class("word")).into_selection().first() {
            let word = word.text();

            let def = panel.find(Class("meaning")).into_selection().first().unwrap().text();
            let defid = panel.attr("data-defid").unwrap_or("").to_owned();

            let example = panel.find(Class("example")).into_selection().first().and_then(|e| Some(e.text()));
            let author = panel.find(Class("author")).into_selection().first().and_then(|e| Some(e.text())).unwrap_or("".to_owned());
            let date = panel.find(Class("author")).into_selection().first().and_then(|e| e.next()).and_then(|e| Some(e.text().trim().to_owned())).unwrap_or("".to_owned());

            let sounds = panel.find(Class("play-sound")).into_selection().first().and_then(|e| e.attr("data-urls").and_then(|s| Some(s.to_owned()))).and_then(|l| Some(json_list_to_strings(&l)));

            let similars = Some(doc.find(Name("ul").and(Class("alphabetical"))).into_selection().find(Name("li")).find(Name("a")).iter().map(|e| e.text()).collect());

            Some(DictDef {
                word: word.trim().to_owned(),
                def: def.trim().to_owned(),
                example: example,
                upvote: 0,
                downvote: 0,
                contributor: author,
                date: date,
                id: defid,
                sounds: sounds,
                similars: similars
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn find_from_url(url: &str) -> Option<DictDef> {
    let client = Client::new();

    let mut resp = client.get(url).send().unwrap();
    let mut body = String::new();
    {
        resp.read_to_string(&mut body).unwrap();
    }

    let dom = Document::from(body.as_str());
    get_def_from_doc(&dom)
}

pub fn find_word_of_the_day() -> Option<DictDef> {
    find_from_url("https://www.urbandictionary.com/")
}

pub fn get_random_word() -> Option<DictDef> {
    find_from_url("https://www.urbandictionary.com/random.php")
}

pub fn find_on_urban_dict (word: &str) -> Option<DictDef> {
    let url = urban_dict_url(word);
    find_from_url(&url)
}
