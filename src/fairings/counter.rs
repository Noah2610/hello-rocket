use super::prelude::*;

pub struct Counter {
    get:  AtomicUsize,
    post: AtomicUsize,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            get:  AtomicUsize::new(0),
            post: AtomicUsize::new(0),
        }
    }
}

impl Fairing for Counter {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Requests Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        match request.method() {
            Method::Get  => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _            => return,
        };
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if response.status() != Status::NotFound {
            return;
        }

        if request.method() == Method::Get && request.uri().path() == "/counts" {
            let get_count  = self.get.load(Ordering::Relaxed);
            let post_count = self.post.load(Ordering::Relaxed);
            let body = format!("Get: {}\nPost: {}", get_count, post_count);
            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(body));
        }
    }
}
