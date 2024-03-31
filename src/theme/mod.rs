use glob::glob;
use handlebars::Handlebars;
use std::{fs::File, io::Read, path::Path};
use tracing::{error, info};

pub static TEMPLATE_INDEX: &[u8] = include_bytes!("index.hbs");
pub static TEMPLATE_RECIPE: &[u8] = include_bytes!("recipe.hbs");

pub struct PrandiumTheme {
    handlebars: Handlebars<'static>,
}

// imlement default
impl Default for PrandiumTheme {
    fn default() -> Self {
        let mut handlebars = Handlebars::new();
        let template_index_string = String::from_utf8_lossy(TEMPLATE_INDEX);
        let template_recipe_string = String::from_utf8_lossy(TEMPLATE_RECIPE);
        handlebars
            .register_template_string("index.hbs", template_index_string)
            .expect("Unable to register template");
        handlebars
            .register_template_string("recipe.hbs", template_recipe_string)
            .expect("Unable to register template");
        PrandiumTheme { handlebars }
    }
}

pub fn load_theme_from_folder(folder: &Path) -> PrandiumTheme {
    info!("Using theme from {}", folder.display());
    let mut theme_files = Vec::new();
    let mut handlebars = Handlebars::new();
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

    PrandiumTheme { handlebars }
}
