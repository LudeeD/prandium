#[derive(Serialize, Deserialize, Debug)]
struct Ingredient {
    name: String,
    description: Option<String>,
    quantity: u32,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    id: String,
    title: String,
    menu_type: String,
    classification: String,
    servings: String,
    time_minutes: String,
    //ingredients: Vec<Ingredient>,
    //instructions: Vec<String>
}
