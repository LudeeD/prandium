use std::{fs::File, io::Write as _, path::PathBuf};

use crate::{
    config::PrandiumConfig,
    parser::{parse_recipe, Recipe},
    theme::PrandiumTheme,
};
use glob::glob;
use serde_json::json;
use tracing::error;

pub struct PrandiumCookbook {
    theme: PrandiumTheme,
    pub config: PrandiumConfig,
    recipes: Vec<Recipe>,
}

impl PrandiumCookbook {
    pub fn new(config: PrandiumConfig, theme: PrandiumTheme) -> Self {
        let recipes = parse_folder();
        PrandiumCookbook {
            theme,
            config,
            recipes,
        }
    }

    pub fn generate(&self) {
        self.write_css();
        self.generate_index_page();
        self.generate_recipe_pages();
    }

    fn write_css(&self) {
        let output_folder = PathBuf::from(self.config.get_output_folder());
        let css_file = output_folder.join("styles.css");
        let mut file = File::create(css_file).expect("Unable to create file");
        file.write_all(self.theme.get_css().as_bytes())
            .expect("Unable to write data");
    }

    fn generate_recipe_pages(&self) {
        let output_folder = self.config.get_output_folder();
        let path = PathBuf::from(output_folder);

        for recipe in self.recipes.iter() {
            let file_path = path
                .clone()
                .join(recipe.id.to_string())
                .with_extension("html");
            let mut file = File::create(file_path).expect("Unable to create file");
            let data = json!({
                "id" : recipe.id,
                "name": recipe.name,
                "ingredients": recipe.ingredients,
                "instructions": recipe.instructions,
            });

            let full_data = json!({
                "config": self.config,
                "recipe": data,
            });

            let rendered = self.theme.render("recipe.hbs", &full_data);
            file.write_all(rendered.as_bytes())
                .expect("Unable to write data");
        }
    }

    fn generate_index_page(&self) {
        let output_folder = PathBuf::from(self.config.get_output_folder());
        let index_file = output_folder.join("index.html");
        let mut file = File::create(index_file).expect("Unable to create file");

        let full_data = json!({
            "config": self.config,
            "recipes": self.recipes,
        });

        let rendered = self.theme.render("index.hbs", &full_data);
        file.write_all(rendered.as_bytes())
            .expect("Unable to write data");
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
