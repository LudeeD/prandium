use glob::glob;
use handlebars::Handlebars;
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

mod config;


use crate::parser::{parse_recipe, Recipe};
mod parser;

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

fn load_config() -> config::PrandiumConfig {
    let working_dir = env::current_dir().unwrap();
    info!("Working directory: {}", working_dir.display());
    let config_path = working_dir.join("prandium.toml");
    if config_path.exists() {
        info!("Config file found: {}", config_path.display());
        config::load_config_from_file(&config_path)
    } else {
        error!("No prandium without a config file!");
        std::process::exit(-1);
    }
}

fn load_theme() -> theme::PrandiumTheme {
    // check if theme folder exists
    let working_dir = env::current_dir().unwrap();
    let theme_path = working_dir.join("theme");
    if theme_path.exists() {
        info!("Using theme from {}", theme_path.display());
        theme::load_theme_from_folder(&theme_path)
    } else {
        warn!("No theme folder found, using default theme");
        theme::PrandiumTheme::default()
    }
}

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    println!("Hello from Prandium");

    let config = load_config();
    config.setup_output_folder();

    let theme = load_theme();

    let recipes = parse_folder();

    //generate_recipe_pages(&mut hbs, &recipes, &config);

    //generate_index_page(&mut hbs, &recipes, &config);
}
