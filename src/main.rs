use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use regex::Regex;

#[derive(Debug, Deserialize, Serialize)]
struct Document {
    pub title: String,
    pub url: String,
    #[serde(rename = "abstract")]
    pub text: String,
    #[serde(skip_deserializing)]
    id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Feed {
    #[serde(rename = "doc")]
    pub docs: Vec<Document>,
}

fn load_documents(path: &str) -> Result<Vec<Document>, Box<dyn Error>> {
    let wiki_doc = fs::read_to_string(path)?;
    let mut feed: Feed = from_str(&wiki_doc)?;

    for (i, mut doc) in feed.docs.iter_mut().enumerate() {
        doc.id = i as u64;
    }

    Ok(feed.docs)
}

fn search(docs: Vec<Document>, term: &str) -> Vec<Document> {
    let mut r: Vec<Document> = Vec::new();

    for doc in docs {
        if doc.text.contains(term) {
            r.push(doc)
        }
    }
    r
}

fn search_regex(docs: Vec<Document>, re: Regex) -> Vec<Document> {
    let mut r: Vec<Document> = Vec::new();

    for doc in docs {
        if re.is_match(&doc.text) {
            r.push(doc);
        }
    }
    r
}

fn main() {
    const PATH: &str = "enwiki-latest-abstract1.xml";

    let docs = match load_documents(PATH) {
        Ok(docs) => docs,
        Err(e) => {
            eprintln!("{:?}", e);
            ::std::process::exit(1);
        }
    };

    // let result = search(docs, "Marine");
    // println!("{:?}", result);

    let re = Regex::new(r#"(?i)\bGoogle\b"#).unwrap();
    let result = search_regex(docs, re);
    println!("{:?}", result);
}