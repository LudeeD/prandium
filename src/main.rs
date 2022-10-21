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
    for entry in glob("./recipes/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let recipe = parse_recipe(path);
                recipes.push(recipe);
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

fn generate_recipe_pages(handlebars: &Handlebars, recipes: &Vec<Recipe>) {
    let output_folder = collect_output_folder_from_args();
    for (i, recipe) in recipes.iter().enumerate() {
        let mut file = File::create(output_folder.join(i.to_string()).with_extension("html"))
            .expect("Unable to create file");
        let data = json!({
            "name": recipe.name,
            "ingredients": recipe.ingredients,
            "instructions": recipe.instructions,
        });
        let rendered = handlebars.render("recipe", &data).unwrap();
        file.write_all(rendered.as_bytes())
            .expect("Unable to write data");
    }
}

fn main() {
    println!("Hello from Prandium");

    let output_path = collect_output_folder_from_args();
    let current_path = env::current_dir().unwrap();

    let mut hbs = Handlebars::new();
    register_templates(&mut hbs);

    let recipes = parse_folder();

    generate_recipe_pages(&mut hbs, &recipes);

    /*
        for path in glob(&g).unwrap().filter_map(Result::ok) {
            let mut output_file = outpath.clone();
            let file_name = path.file_name().expect("TODO");

            if file_name == "README.md" {
                continue;
            };
            let filename = path.clone();
            let metadata = fs::metadata(path.clone()).unwrap();

            println!("Found {:?} -> {}", file_name, recipe_id);
            let recipe = parse_recipe(path);
            let title = recipe.name;
            let time = metadata.modified().unwrap();

            let filename = filename.file_name().unwrap().to_owned();

            let render = hbs
                .render("recipe", &json!({ "ingredients":{}, "instructions": {} }))
                .unwrap();

            output_file.push(recipe_id.to_string());
            output_file.set_extension("html");
            let mut file = File::create(output_file.clone()).unwrap();
            file.write_all(render.as_bytes()).expect("TODO");

            let info = output_file
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            index_files.push((info, title, time));

            recipe_id += 1;
        }


        let mut index_file = outpath.clone();
        index_file.push("index");
        index_file.set_extension("html");

        let demo: Vec<JsonValue> = index_files
            .iter()
            .map(|nota| json!({"title": nota.1, "link": nota.0}))
            .collect();

        let render = hbs
            .render("index", &json!({ "recipes": demo }))
            .expect("TODO");

        let mut file = File::create(index_file).unwrap();
        file.write_all(&render.as_bytes()).expect("TODO");
    */
}
