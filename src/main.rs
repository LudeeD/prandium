use clap::Parser;
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::io;

use serde::{Serialize, Deserialize};
use serde_json;
use uuid::Uuid;
use sha1::{Sha1, Digest};

#[derive(Serialize, Deserialize, Debug)]
struct Ingredient {
    name: String,
    description: Option<String>,
    quantity: u32,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    id: String,
    title: String,
    menu_type: String,
    classification: String,
    servings: String,
    time_minutes: String,
    //ingredients: Vec<Ingredient>,
    //instructions: Vec<String>
}

#[derive(Parser, Debug)]
#[clap(name = "prandium")]
struct Opts {
    /// URL to parse
    url: String,
}

fn main() {
    println!("Hello, world!");

    let hello = Opts::parse();

    println!("{}", hello.url);

    pingo_doce_parser(&hello.url);

}

fn pingo_doce_parser(url: &str) {

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();


    let mut recipe = None;

    // same as before
    for node in document.find(Class("recipe-page")) {
        let title = node.find(Class("recipe-title")).next().unwrap().text();
        println!("Found Recipe : {}", title);

        let classification = String::from("vegetariano");
        let menu_type = String::from("refeição");
        // let mut classification = String::with_capacity(10);
        // println!("Classificação[vegetariano, carne, peixe]:");
        // io::stdin().read_line(&mut classification).expect("Error reading input");
        // classification.truncate(classification.trim_end().len());


        let mut servings = String::new();
        let mut time_minutes = String::new();
        for (i,items) in node.find(Class("info")).enumerate() {
            match i {
                0 => time_minutes = items.text(),
                2 => servings = items.text(),
                _ => {}
            }
        }
        println!("porções: {}", servings);
        println!("tempo: {}", time_minutes);

        let mut hasher = Sha1::new();
        // process input message
        hasher.update(title.clone().into_bytes());

        let hash = hasher.finalize();

        let id = format!("{:x}", hash);

        recipe = Some(
            Recipe{
                id,
                title,
                menu_type,
                classification,
                servings,
                time_minutes,
            }
        )
    }


    let recipe = recipe.unwrap();
    let serialized = serde_json::to_string(&recipe).unwrap();


    println!("{}", serialized);

}
