use hyper::Client;
use kuchiki::{Html, NodeRef, NodeDataRef, ElementData};

pub struct DictDef {
    pub word: String,
    pub def: String
}

impl DictDef {
    pub fn new (word: String, def: String) -> DictDef {
        DictDef {
            word: word,
            def: def
        }
    }
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

pub fn find_on_urban_dict (word: &str) -> Option<DictDef> {
    let url = urban_dict_url(word);

    let client = Client::new();

    let mut response = client.get(&url).send().unwrap();

    if let Ok(html) = Html::from_stream(&mut response) {
        let doc = html.parse();

        if let Some(panel_ele) = get_first_match(&doc, "div.def-panel") {
            let panel = panel_ele.as_node();
            let word_ele = get_first_match(panel, "a.word").unwrap();
            let word_node = word_ele.as_node();
            let word = word_node.first_child().unwrap();
            let word_text = word.as_text().unwrap().borrow();

            let def_ele = get_first_match(panel, "div.meaning").unwrap();
            let def_node = def_ele.as_node();
            let def = def_node.first_child().unwrap();
            let def_text = def.as_text().unwrap().borrow();

            return Some(DictDef::new(word_text.trim().to_owned(),
                                     def_text.trim().to_owned()));
        }
    }

    None
}
