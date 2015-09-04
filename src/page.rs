use hyper::Client;
use kuchiki::{Html, NodeRef, NodeDataRef, ElementData};
use string_cache::namespace::{Namespace, QualName};
use string_cache::atom::Atom;

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
    pub sounds: Vec<String>
}

impl DictDef {
}

fn urban_dict_url (word: &str) -> String {
    let mut s = String::new();
    s.push_str("http://www.urbandictionary.com/define.php?term=");
    s.push_str(word);
    s
}

fn get_first_match(node: &NodeRef, selector: &str) -> Option<NodeDataRef<ElementData>> {
    if let Ok(mut eles) = node.select(selector) {
        eles.next()
    } else {
        None
    }
}

fn get_attribute_from_node(panel: &NodeRef, attr_name: &str) -> Option<String> {
    if let Some(panel_ele) = panel.as_element() {
        let attr_map = panel_ele.attributes.borrow();
        if let Some(attr_value) = attr_map.get(
            &QualName::new(Namespace(Atom::from_slice("")), Atom::from_slice(attr_name))) {
            return Some(attr_value.clone());
        }
    }
    None
}

fn get_node_text(node_data: &NodeDataRef<ElementData>) -> String {
    let node = node_data.as_node();
    node.text_contents()
}

fn json_list_to_strings(json: &str) -> Vec<String> {
    json[1..json.len()-1].split(", ").map(|s| s[1..s.len()-1].to_owned()).collect()
}

fn get_def_from_page_ele (panel: &NodeRef) -> Option<DictDef> {
    if let Some(word_ele) = get_first_match(panel, "a.word") {
        let word_text = get_node_text(&word_ele);

        let def_ele = get_first_match(panel, "div.meaning").unwrap();
        let def_text = get_node_text(&def_ele);

        let defid = get_attribute_from_node(panel, "data-defid").unwrap_or("".to_owned());

        let example = if let Some(example_ele) = get_first_match(panel, "div.example") {
            Some(get_node_text(&example_ele))
        } else {
            None
        };

        let author_ele = get_first_match(panel, "a.author").unwrap();
        let author = get_node_text(&author_ele);
        let date_node = author_ele.as_node().next_sibling().unwrap();
        let date_text = date_node.as_text().unwrap().borrow().trim().to_owned();

        let sounds_ele = get_first_match(panel, "a.play-sound").unwrap();
        let sounds_node = sounds_ele.as_node();
        let sounds_json_list = get_attribute_from_node(&sounds_node, "data-urls").unwrap_or("[]".to_owned());
        let sounds = json_list_to_strings(&sounds_json_list);

        Some(DictDef {
            word: word_text.trim().to_owned(),
            def: def_text.trim().to_owned(),
            example: example,
            upvote: 0,
            downvote: 0,
            contributor: author,
            date: date_text,
            id: defid,
            sounds: sounds
        })
    } else {
        None
    }
}

fn find_from_url(url: &str) -> Option<DictDef> {
    let client = Client::new();

    let mut response = client.get(url).send().unwrap();

    if let Ok(html) = Html::from_stream(&mut response) {
        let doc = html.parse();

        if let Some(panel_ele) = get_first_match(&doc, "div.def-panel") {
            let panel = panel_ele.as_node();
            return get_def_from_page_ele(panel);
        }
    }

    None
}

pub fn find_word_of_the_day() -> Option<DictDef> {
    find_from_url("http://www.urbandictionary.com/")
}

pub fn find_on_urban_dict (word: &str) -> Option<DictDef> {
    let url = urban_dict_url(word);
    find_from_url(&url)
}
