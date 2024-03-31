use serde::{Deserialize, Serialize};
use std::path::Path;
use toml;

#[derive(Deserialize, Serialize)]
pub struct PrandiumConfig {
    title: String,
    base_url: String,
    author: String,
    author_url: String,
    output_folder: String,
    description: String,
    translations: Translations,
}

#[derive(Deserialize, Serialize)]
pub struct Translations {
    ingredients: String,
    instructions: String,
}

// implement a default for the PrandiumConfig struct
impl Default for PrandiumConfig {
    fn default() -> Self {
        PrandiumConfig {
            title: "Prandium".to_string(),
            base_url: "http://localhost:8765".to_string(),
            author: "Prandium".to_string(),
            author_url: "".to_string(),
            output_folder: "output".to_string(),
            description: "A simple recipe site".to_string(),
            translations: Translations {
                ingredients: "Ingredients".to_string(),
                instructions: "Instructions".to_string(),
            },
        }
    }
}

impl PrandiumConfig {
    pub fn setup_output_folder(&self) {
        let output_folder = Path::new(&self.output_folder);
        if !output_folder.exists() {
            std::fs::create_dir_all(&output_folder).unwrap();
        }
    }

    pub fn get_output_folder(&self) -> &str {
        &self.output_folder
    }
}

pub fn load_config_from_file(file: &Path) -> PrandiumConfig {
    let str_config = std::fs::read_to_string(file).unwrap_or_default();
    let config = toml::from_str(&str_config).unwrap_or_default();
    config
}

pub fn initialize_folder() {
    let working_dir = std::env::current_dir().unwrap();
    println!("Initializing Prandium in {}", working_dir.display());
    // ask for base url
    println!("Enter the base URL of your site:");
    let mut base_url = String::new();
    std::io::stdin().read_line(&mut base_url).unwrap();
    // ask for title
    println!("Enter the title of your site:");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();
    // ask for author
    println!("Enter the author of your site:");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();
    // ask for author url
    println!("Enter the author's URL:");
    let mut author_url = String::new();
    std::io::stdin().read_line(&mut author_url).unwrap();
    // ask for description
    println!("Enter the description of your site:");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).unwrap();
    // ask for output folder
    println!("Enter the output folder:");
    let mut output_folder = String::new();
    std::io::stdin().read_line(&mut output_folder).unwrap();
    // ask for translations
    println!("Enter the translation for ingredients:");
    let mut ingredients = String::new();
    std::io::stdin().read_line(&mut ingredients).unwrap();
    println!("Enter the translation for instructions:");
    let mut instructions = String::new();
    std::io::stdin().read_line(&mut instructions).unwrap();

    let config = PrandiumConfig {
        title: title.trim().to_string(),
        base_url: base_url.trim().to_string(),
        author: author.trim().to_string(),
        author_url: author_url.trim().to_string(),
        output_folder: output_folder.trim().to_string(),
        description: description.trim().to_string(),
        translations: Translations {
            ingredients: ingredients.trim().to_string(),
            instructions: instructions.trim().to_string(),
        },
    };

    let config_str = toml::to_string(&config).unwrap();
    let config_path = working_dir.join("prandium.toml");
    std::fs::write(config_path, config_str).unwrap();
}
