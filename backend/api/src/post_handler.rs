use application::post::{create, delete, read};
use domain::models::post::{Post, PostForm, PostReturn};
use rocket::form::Form;
use rocket::response::status::{Created, NotFound};
use rocket::{get, post};
use shared::response_models::{Response, ResponseBody};

#[get("/list_posts")]
pub fn list_posts_handler() -> String {
    let posts: Vec<PostReturn> = read::list_posts().expect("Unable to retrieve posts.");
    let response = Response {
        body: ResponseBody::PostReturns(posts),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::list_post(post_id)?;
    let response = Response {
        body: ResponseBody::Post(post),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_post", format = "multipart/form-data", data = "<post>")]
pub fn create_post_handler(
    post: Form<PostForm>,
) -> Result<Created<String>, rocket::response::status::Conflict<String>> {
    println!("{:?}", post);
    create::create_post::<Post>(post)
}

//#[post("/upload", format = "multipart/form-data", data = "<media>")]
//pub fn upload_media_handler<'f>(media: Form<TempFile<'f>>) -> Created<String> {
//    println!("{:?}", media);
//    //upload::upload_media(media)
//}

#[get("/delete/<post_id>")]
pub fn delete_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let posts = delete::delete_post(post_id)?;
    let response = Response {
        body: ResponseBody::Posts(posts),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
