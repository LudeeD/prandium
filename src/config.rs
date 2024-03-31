use serde::Deserialize;
use std::{fs::File, path::Path};
use toml;

#[derive(Deserialize)]
pub struct PrandiumConfig {
    title: String,
    base_url: String,
    author: String,
    author_url: String,
    output_folder: String,
    description: String,
    translations: Translations,
}

#[derive(Deserialize)]
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
}

pub fn load_config_from_file(file: &Path) -> PrandiumConfig {
    let str_config = std::fs::read_to_string(file).unwrap_or_default();
    let config = toml::from_str(&str_config).unwrap_or_default();
    config
}
