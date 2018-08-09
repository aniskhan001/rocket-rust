#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/<some>")]
fn bello(some: String) -> String {
    format!("Bello, {}!", some.to_uppercase())
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![hello])
        .mount("/bello", routes![bello])
        .launch();
}
