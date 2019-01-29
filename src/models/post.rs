use rocket::request::Form;
use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id:        i32,
    pub title:     String,
    pub body:      String,
    pub published: bool,
}

#[derive(FromForm)]
pub struct PostData {
    title:     String,
    body:      String,
    published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title:     String,
    pub body:      String,
    pub published: bool,
}

impl From<Form<PostData>> for NewPost {
    fn from(data: Form<PostData>) -> Self {
        Self {
            title:     data.title.clone(),
            body:      data.body.clone(),
            published: data.published,
        }
    }
}
