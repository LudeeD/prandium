use glob::glob;
use handlebars::Handlebars;
use handlebars::JsonValue;
use serde_json::json;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

mod theme;
use theme::{TEMPLATE_INDEX, TEMPLATE_RECIPE};

use crate::parser::{parse_recipe, Recipe};
mod parser;

// register index and recipe template on handlebars
fn register_templates(handlebars: &mut Handlebars) {
    let template_index_string = String::from_utf8_lossy(TEMPLATE_INDEX);
    let template_recipe_string = String::from_utf8_lossy(TEMPLATE_RECIPE);
    handlebars
        .register_template_string("index", template_index_string)
        .unwrap();
    handlebars
        .register_template_string("recipe", template_recipe_string)
        .unwrap();
}

// iterate over all recipes and generate a list of recipes
fn parse_folder() -> Vec<Recipe> {
    let mut recipes = Vec::new();
    let mut next_id = 0;
    for entry in glob("./recipes/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let recipe = parse_recipe(path, next_id);
                recipes.push(recipe);
                next_id += 1;
            }
            Err(e) => println!("{:?}", e),
        }
    }
    recipes
}

fn collect_output_folder_from_args() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    let output_folder = match args.len() {
        1 => PathBuf::from("./docs"),
        2 => PathBuf::from(&args[1]),
        _ => panic!("Too many arguments"),
    };
    output_folder
}

fn generate_recipe_pages(
    handlebars: &Handlebars,
    recipes: &Vec<Recipe>,
    config: &serde_json::Value,
) {
    let output_folder = PathBuf::from(config["output_folder"].as_str().unwrap());

    // check if output folder exists
    if !output_folder.exists() {
        fs::create_dir_all(&output_folder).unwrap();
    }

    for recipe in recipes {
        let mut file = File::create(
            output_folder
                .join(recipe.id.to_string())
                .with_extension("html"),
        )
        .expect("Unable to create file");
        let data = json!({
            "name": recipe.name,
            "ingredients": recipe.ingredients,
            "instructions": recipe.instructions,
        });

        let full_data = json!({
            "config": config,
            "recipe": data,
        });

        let rendered = handlebars.render("recipe", &full_data).unwrap();
        file.write_all(rendered.as_bytes())
            .expect("Unable to write data");
    }
}

fn generate_index_page(handlebars: &Handlebars, recipes: &Vec<Recipe>, config: &serde_json::Value) {
    let output_folder = PathBuf::from(config["output_folder"].as_str().unwrap());

    // check if output folder exists
    if !output_folder.exists() {
        fs::create_dir_all(&output_folder).unwrap();
    }

    let mut file = File::create(output_folder.join("index.html")).expect("Unable to create file");

    let full_data = json!({
        "config": config,
        "recipes": recipes,
    });

    let rendered = handlebars.render("index", &full_data).unwrap();
    file.write_all(rendered.as_bytes())
        .expect("Unable to write data");
}

fn read_config() -> JsonValue {
    let mut file = File::open("./config.json").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read string");
    let config: JsonValue = serde_json::from_str(&contents).unwrap();
    config
}

fn check_config_file_present() {
    if !PathBuf::from("./config.json").exists() {
        println!("Do you want to create a config file? [y/n]");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() != "y" {
            println!("No prandium without a config file!");
            std::process::exit(0);
        }
        let mut file = File::create("./config.json").expect("Unable to create file");
        let config = json!({
            "title": "My Cookbook",
            "author": "John Doe",
            "author_url": "https://google.com",
            "description": "A cookbook for my recipes",
            "translations" : {
                "ingredients": "Ingredients",
                "instructions": "Instructions",
            }
        });
        let config_string = serde_json::to_string_pretty(&config).unwrap();
        file.write_all(config_string.as_bytes())
            .expect("Unable to write data");
    }
}

fn main() {
    println!("Hello from Prandium");

    check_config_file_present();

    let today_date = chrono::Local::today().format("%Y-%m-%d").to_string();

    let mut config = read_config();
    config["date"] = json!(today_date);

    //let output_path = collect_output_folder_from_args();
    let output_path = PathBuf::from(config["output_folder"].as_str().unwrap());
    let current_path = env::current_dir().unwrap();

    let mut hbs = Handlebars::new();
    register_templates(&mut hbs);

    let recipes = parse_folder();

    generate_recipe_pages(&mut hbs, &recipes, &config);

    generate_index_page(&mut hbs, &recipes, &config);
}
