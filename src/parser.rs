use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct Recipe {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub instructions: Vec<String>,
    pub ingredients: Vec<String>,
}

pub fn parse_recipe(path: PathBuf) -> Recipe {
    lazy_static! {
        static ref RE_TITL: Regex = Regex::new("^# .*$").unwrap();
        static ref RE_ATTR: Regex = Regex::new("^\\* .*$").unwrap();
        static ref RE_INGR: Regex = Regex::new("^\\- .*$").unwrap();
        static ref RE_INST: Regex = Regex::new("^\\+ .*$").unwrap();
    }
    println!("Parsing {}", path.display());

    let mut name = String::new();
    let attributes = Vec::new();
    let mut instructions = Vec::new();
    let mut ingredients = Vec::new();

    if let Ok(lines) = read_lines(path.as_path()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if RE_TITL.is_match(&ip) {
                    name = ip.replace("#", "").trim().to_string();
                } else if RE_ATTR.is_match(&ip) {
                } else if RE_INST.is_match(&ip) {
                    let instruction = ip.replace("+", "").trim().to_string();
                    instructions.push(instruction);
                } else if RE_INGR.is_match(&ip) {
                    let ingredient = ip.replace("-", "").trim().to_string();
                    ingredients.push(ingredient);
                }
            }
        }
    }

    Recipe {
        name,
        attributes,
        ingredients,
        instructions,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
