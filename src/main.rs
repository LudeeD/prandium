use glob::glob;
use handlebars::Handlebars;
use handlebars::JsonValue;
use serde_json::json;
use tracing::info;
use tracing::error;
use tracing::warn;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

mod theme;
use theme::{TEMPLATE_INDEX, TEMPLATE_RECIPE};

use crate::parser::{parse_recipe, Recipe};
mod parser;

// check if theme folder exists, if it does register handlerbars from it, otherwise use default theme
fn register_theme(handlebars: &mut Handlebars) {
    let theme_path = PathBuf::from("theme");
    if theme_path.exists() {
        info!("Using theme from {}", theme_path.display());
        let mut theme_files = Vec::new();
        for entry in glob("theme/*.hbs").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    theme_files.push(path);
                }
                Err(e) => error!("{:?}", e),
            }
        }
        for file in theme_files.iter() {
            let mut f = File::open(file).expect("Unable to open file");
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("Unable to read string");
            info!(
                "Registering template {}",
                file.file_name().unwrap().to_str().unwrap()
            );
            handlebars
                .register_template_string(file.file_name().unwrap().to_str().unwrap(), contents)
                .expect("Unable to register template");
        }
    } else {
        warn!("No theme folder found, using default theme");
        let template_index_string = String::from_utf8_lossy(TEMPLATE_INDEX);
        let template_recipe_string = String::from_utf8_lossy(TEMPLATE_RECIPE);
        handlebars
            .register_template_string("index.hbs", template_index_string)
            .expect("Unable to register template");
        handlebars
            .register_template_string("recipe.hbs", template_recipe_string)
            .expect("Unable to register template");
    }
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
            Err(e) => error!("{:?}", e),
        }
    }
    recipes
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
            "id" : recipe.id,
            "name": recipe.name,
            "ingredients": recipe.ingredients,
            "instructions": recipe.instructions,
        });

        let full_data = json!({
            "config": config,
            "recipe": data,
        });

        let rendered = handlebars.render("recipe.hbs", &full_data).unwrap();
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

    let rendered = handlebars.render("index.hbs", &full_data).unwrap();
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
            error!("No prandium without a config file!");
            std::process::exit(0);
        }
        let mut file = File::create("./config.json").expect("Unable to create file");
        let config = json!({
            "title": "My Cookbook",
            "base_url": "https://example.com",
            "author": "John Doe",
            "author_url": "https://google.com",
            "output_folder": "./docs",
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

fn create_output_folder(config: &serde_json::Value) {
    let output_folder = PathBuf::from(config["output_folder"].as_str().unwrap());
    if !output_folder.exists() {
        fs::create_dir_all(&output_folder).unwrap();
    }
}

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    println!("Hello from Prandium");

    check_config_file_present();

    let today_date = chrono::Local::today().format("%Y-%m-%d").to_string();
    let mut config = read_config();
    config["date"] = json!(today_date);

    create_output_folder(&config);

    let mut hbs = Handlebars::new();
    register_theme(&mut hbs);

    let recipes = parse_folder();

    generate_recipe_pages(&mut hbs, &recipes, &config);

    generate_index_page(&mut hbs, &recipes, &config);
}
