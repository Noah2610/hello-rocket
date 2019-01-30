pub mod prelude {
    pub use std::io::Cursor;
    pub use std::sync::atomic::{ AtomicUsize, Ordering };
    pub use rocket::fairing::{ Fairing, Info, Kind };
    pub use rocket::Request;
    pub use rocket::Response;
    pub use rocket::Data;
    pub use rocket::http::{ Status, Method, ContentType };
}

pub mod counter;

pub use self::counter::Counter;
