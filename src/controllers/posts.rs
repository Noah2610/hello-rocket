use super::prelude::*;

#[get("/posts")]
pub fn index(db: DbConnection) -> content::Html<String> {
    use crate::schema::posts::dsl::*;

    let psts = posts.load::<Post>(&db as &PgConnection).expect("Should load posts");
    content::Html (
        (html! {
            (header(Some("Posts")));
            (navigation(Some("/posts")));
            div.container {
                h1 { "Posts" }
                (PreEscaped(psts.iter()
                            .map( post_entry ).collect::<String>()));
            };
            (footer());
        }).into_string()
    )
}

fn post_entry(post: &Post) -> String {
    (html! {
        div.panel.panel-default {
            div.panel-heading { (post.title) }
            div.panel-body    { (post.body)  }
            div.panel-footer  { "Published: " strong { (post.published) } }
        }
    }).into_string()
}

#[get("/posts/new")]
pub fn new() -> content::Html<String> {
    content::Html((html! {
        (header(Some("New Post")));
        (navigation(Some("/posts/new")));
        div.container {
            h1 { "Create a new Post!" }
            form action="/posts/create" method="post" {
                label.control-label for="title" { "Post Title" };
                input.form-control type="text" name="title" placeholder="Post Title" {};
                label.control-label for="body" { "Post Content" };
                textarea.form-control name="body" placeholder="Post Content" {};
                input.control-label type="checkbox" name="published" {};
                label.control-label for="published" { "Publish post" };
                br;
                input.btn.btn-primary type="submit" value="Create Post!" { };
            }
        }
    }).into_string())
}

#[post("/posts/create", data = "<post_data>")]
pub fn create(db: DbConnection, post_data: Form<PostData>) -> Redirect {
    use crate::schema::posts;

    let new_post = NewPost::from(post_data);
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&db as &PgConnection)
        .expect("Error saving new post");
    Redirect::to("/posts")
}
