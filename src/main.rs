#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde;

#[derive(serde::Serialize, serde::Deserialize)]
struct Item {
    id: i32,
    name: String,
    description: Option<String>,
    price: i32
}


#[post("/", data = "<an_item>")]
fn create_item(an_item: Json<Item>) -> Json<Item> {
    an_item
}

#[get("/")]
fn list_item() -> Json<Vec<Item>> {
    Json(vec![
        Item {
            id: 1,
            name: "item1".to_string(),
            description: Some("description1".to_string()),
            price: 100,
        },
        Item {
            id: 2,
            name: "item2".to_string(),
            description: Some("description2".to_string()),
            price: 200,
        },
    ])
}

#[put("/<_id>", data = "<an_item>")]
fn update_item(_id: i32, an_item: Json<Item>) -> Json<Item> {
    an_item
}

#[delete("/<_id>")]
fn delete_item(_id: i32) -> status::NoContent {
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/item", routes![create_item, update_item, delete_item])
        .mount("/items", routes![list_item])
}
