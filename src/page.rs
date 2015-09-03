use hyper::Client;
use kuchiki::{Html, NodeRef, NodeDataRef, ElementData};

#[derive(Debug)]
pub struct DictDef {
    pub word: String,
    pub def: String,
    pub example: Option<String>
}

impl DictDef {
    pub fn new (word: String, def: String, example: Option<String>) -> DictDef {
        DictDef {
            word: word,
            def: def,
            example: example
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

fn get_def_from_page_ele (panel: &NodeRef) -> Option<DictDef> {
    if let Some(word_ele) = get_first_match(panel, "a.word") {
        let word_node = word_ele.as_node();
        let word_text = word_node.text_contents();

        let def_ele = get_first_match(panel, "div.meaning").unwrap();
        let def_node = def_ele.as_node();
        let def_text = def_node.text_contents();

        let example = if let Some(example_ele) = get_first_match(panel, "div.example") {
            let example_node = example_ele.as_node();
            Some(example_node.text_contents())
        } else {
            None
        };

        Some(DictDef::new(word_text.trim().to_owned(),
                          def_text.trim().to_owned(),
                          example))
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
            return get_def_from_page_ele(panel);
        }
    }

    None
}
