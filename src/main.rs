#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod model;
use model::{Item};

#[post("/", data = "<an_item>")]
fn create_item(an_item: Json<Item>) -> Json<Item> {
    an_item
}

#[get("/")]
fn list_item() -> Json<Value> {
    Json(json!([
        {
            "name": "a disk",
            "description": "useful item to store information",
            "price": 16
        },
        {
            "name": "b disk",
            "description": "another useful item",
            "price": 13
        }
    ]))
}

#[put("/<_id>", data = "<an_item>")]
fn update_item(_id: i32, an_item: Json<Item>) -> Json<Item> {
    an_item
}

#[delete("/<_id>")]
fn delete_item(_id: i32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite()
        .mount("/item", routes![create_item, update_item, delete_item])
        .mount("/items", routes![list_item])
        .launch();
}
