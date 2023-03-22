use rocket::serde::json::Json;

use crate::models::Model;
pub mod models;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/random")]
fn random_model() -> Json<Model> {
    Json(Model {
        id: 1,
        name: "haha".to_string(),
        path: "path".to_string(),
    })
}

#[post("/", data = "<model>")]
fn create_model(model: Json<Model>) -> Json<Model> {
    model
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, random_model, create_model])
}
