use std::env;
use tracing::error;
use tracing::info;
use tracing::warn;
mod config;
mod cookbook;
mod theme;
use cookbook::PrandiumCookbook;
mod parser;
mod server;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    init: bool,

    #[arg(long)]
    server: bool,
}

fn load_config() -> config::PrandiumConfig {
    let working_dir = env::current_dir().unwrap();
    info!("Working directory: {}", working_dir.display());
    let config_path = working_dir.join("prandium.toml");
    if config_path.exists() {
        info!("Config file found: {}", config_path.display());
        config::load_config_from_file(&config_path)
    } else {
        error!("No prandium without a config file! try running `prandium init` first.");
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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    if args.init {
        info!("Initializing folder");
        config::initialize_folder();
    }

    info!("Hello from Prandium");

    let config = load_config();
    config.setup_output_folder();

    let theme = load_theme();

    let cookbook = PrandiumCookbook::new(config, theme);

    cookbook.generate();

    if args.server {
        info!("Starting server");
        server::start(cookbook).await;
    }


    //generate_recipe_pages(&mut hbs, &recipes, &config);

    //generate_index_page(&mut hbs, &recipes, &config);
}
