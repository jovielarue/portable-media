use diesel::prelude::*;
use domain::models::{Post, PostForm};
use infrastructure::establish_connection;
use rocket::{form::Form, response::status::Created};
use shared::response_models::{Response, ResponseBody};

pub fn create_post(new_post: Form<PostForm>) -> Created<String> {
    use domain::schema::posts;

    let new_post = new_post.into_inner();
    println!("{:?}", new_post);

    let post: Post = Post {
        post_id: 1,
        description: new_post.description,
        song: new_post.song,
        like_count: None,
    };

    match diesel::insert_into(posts::table)
        .values(&post)
        .get_result::<Post>(&mut establish_connection())
    {
        Ok(post) => {
            let response = Response {
                body: ResponseBody::Post(post),
            };
            println!("Db response - {:?}", response);
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
