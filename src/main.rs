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

fn main() {
    rocket::ignite()
        .mount("/", controllers::get_routes())
        .attach(db::DbConnection::fairing())
        .attach(fairings::Counter::new())
        .launch();
}
