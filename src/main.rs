use glob::glob;
use handlebars::Handlebars;
use serde_json::json;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tracing::error;
use tracing::info;
use tracing::warn;
mod theme;
use theme::{TEMPLATE_INDEX, TEMPLATE_RECIPE};
mod config;
mod cookbook;
use cookbook::PrandiumCookbook;
mod parser;

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

    let cookbook = PrandiumCookbook::new(config, theme);

    cookbook.generate();

    //generate_recipe_pages(&mut hbs, &recipes, &config);

    //generate_index_page(&mut hbs, &recipes, &config);
}
