use glob::glob;
use handlebars::Handlebars;
use serde_json::json;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

mod theme;
use theme::TEMPLATE_INDEX;

fn main() {
    println!("Hello from Prandium");

    let args: Vec<String> = env::args().collect();
    let outpath = args.get(1);
    if outpath.is_none() {
        println!("No outpath provided");
        return;
    }
    let outpath = PathBuf::from(outpath.unwrap());
    let path = env::current_dir().unwrap();

    let mut hbs = Handlebars::new();

    hbs.register_template_string(
        "index",
        String::from_utf8(TEMPLATE_INDEX.to_vec()).expect("TODO"),
    )
    .unwrap();

    hbs.register_template_string(
        "recipe",
        String::from_utf8(TEMPLATE_RECIPE.to_vec()).expect("TODO"),
    )
    .unwrap();

    let g = format!("{}/**/*.md", path.display());
    let mut index_files = Vec::new();

    let mut recipe_id = 1;
    for path in glob(&g).unwrap().filter_map(Result::ok) {
        let mut output_file = outpath.clone();
        let file_name = path.file_name().expect("TODO");

        if file_name == "README" {
            continue;
        };

        print!("Found {:?} ", file_name);
        //let title = parse(&path)?;
        let title = "Comida dormida".to_string();
        let metadata = fs::metadata(file_name).unwrap();
        let time = metadata.modified().unwrap();

        let filename = path.clone();
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

    let render = hbs.render("index", &json!({})).expect("TODO");

    let mut file = File::create(index_file).unwrap();
    file.write_all(&render.as_bytes()).expect("TODO");
}
