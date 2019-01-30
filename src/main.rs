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
use rocket::fairing::AdHoc;

pub struct ResourcesDir(String);

fn main() {
    rocket()
        .launch();
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", controllers::get_routes())
        .attach(db::DbConnection::fairing())
        .attach(fairings::Counter::new())
        .attach(AdHoc::on_attach("Resources Config", |rocket| {
            let res_dir = rocket.config()
                .get_str("resources_dir")
                .unwrap_or("resources/")
                .to_string();
            Ok(rocket.manage(ResourcesDir(res_dir)))
        }))
}
