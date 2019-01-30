pub mod prelude {
    pub use rocket::http::Status;
    pub use rocket::request::Form;
    pub use rocket::response::{ Redirect, content };
    pub use rocket::Route;
    pub use rocket::State;
    pub use rocket_contrib::databases::diesel::prelude::*;
    pub use rocket_contrib::databases::diesel::PgConnection;
    pub use maud::{ DOCTYPE, Markup, PreEscaped };
    pub use crate::db::DbConnection;
    pub use crate::models::*;
    pub use super::layouts::prelude::*;
}

pub mod layouts;
pub mod posts;

use std::path::{ Path, PathBuf };
use rocket::response::NamedFile;

use self::prelude::*;
use super::ResourcesDir;

pub fn get_routes() -> Vec<Route> {
    routes![
        resource,
        index,
        hello,
        posts::index,
        posts::new,
        posts::create,
    ]
}

#[get("/res/<file..>")]
pub fn resource(file: PathBuf, res_dir: State<ResourcesDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&res_dir.0).join(file)).ok()
}

#[get("/")]
pub fn index() -> content::Html<String> {
    content::Html ((html! {
        (header(None));
        (navigation(Some("/")));
        h1 { "Hello Rocket!" };
        (footer());
    }).into_string())
}

#[get("/hello")]
pub fn hello() -> String {
    String::from("Hello Rocket!")
}

#[cfg(test)]
mod tests;
