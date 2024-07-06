use libs::time;
use rocket::{get, launch, routes}; 
mod libs;

#[get("/")]
fn pepe() -> &'static str {
    "pepe"
}

#[get("/adios")]
fn adios() -> &'static str {
    "Adios"
}

#[get("/hola")]
fn hola() -> &'static str {
    time::get_hola()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![pepe, adios, hola])
}