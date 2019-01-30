#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate maud;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod schema;
mod db;
mod controllers;
mod models;
mod fairings;

use rocket::Rocket;

fn main() {
    rocket()
        .attach(db::DbConnection::fairing())
        .attach(fairings::Counter::new())
        .launch();
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", controllers::get_routes())
}
