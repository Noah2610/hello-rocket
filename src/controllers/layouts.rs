pub mod prelude {
    pub use super::header;
    pub use super::navigation;
    pub use super::footer;
}

use maud::{ DOCTYPE, Markup };

pub fn header(title_opt: Option<&str>) -> Markup {
    html! {
        (DOCTYPE);
        head {
            meta charset="utf-8";
            title { (compose_title(title_opt)) }
            link rel="stylesheet" type="text/css" href="/res/lib/css/bootstrap.min.css";
            script type="text/javascript" src="/res/lib/js/bootstrap.min.js" {}
        }
    }
}

fn compose_title(title_opt: Option<&str>) -> String {
    const PAGE_TITLE: &str = "Hello Rocket";
    if let Some(title) = title_opt {
        format!("{} | {}", title, PAGE_TITLE)
    } else {
        PAGE_TITLE.to_string()
    }
}

pub fn navigation(current_uri: Option<&str>) -> Markup {
    html! {
        div.container {
            ul.nav.nav-pills {
                li.active[is_uri(current_uri, "/")] {
                    a href="/" { "Home" }
                }
                li.active[is_uri(current_uri, "/one")] {
                    a href="/posts" { "Posts" }
                }
                li.active[is_uri(current_uri, "/two")] {
                    a href="/posts/new" { "New Post" }
                }
            }
        }
    }
}

fn is_uri(current_uri: Option<&str>, to_check: &str) -> bool {
    if let Some(uri) = current_uri {
        uri == to_check
    } else {
        false
    }
}

pub fn footer() -> Markup {
    html! { }
}
