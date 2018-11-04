#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: i32) -> String {
    let number = get_number();
    format!("Hello, {} year old named {}!", age + number, name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}

fn get_number() -> i32 {
    4
}
