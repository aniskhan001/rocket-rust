#[derive(Serialize, Deserialize)]
pub struct Item {
    id: Option<i32>,
    name: String,
    description: Option<String>,
    price: i32
}
