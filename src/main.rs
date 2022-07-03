#[macro_use] extern crate rocket;

//I FUCKING LOVE ROCKET
use rocket::Shutdown;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite_db")]
struct Db(sqlx::SqlitePool);

struct SUAuth<'r>(&'r str);

#[derive(Debug)]
enum KeyError {
	Missing,
	Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SUAuth<'r> {
    type Error = KeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::Forbidden, KeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(SUAuth(key)),
            Some(_) => Outcome::Failure((Status::Forbidden, KeyError::Invalid)),
        }
    }
}

#[get("/shutdown")]
fn shutdown( shutdown: Shutdown, _suauth: SUAuth ) -> Status {
	shutdown.notify();
	Status::Ok
}

#[get( "/" )]
fn index() -> Status {
	Status::Ok
}

// User Functions

//Return 200 if User is found, else return 404
#[get( "/user/<id>" )]
fn user(id: i32) -> Status {
	
	if id != 69 {
		Status::NotFound
	} else {
		Status::Ok
	}
}

//Get all yeahed posts from user
#[get( "/user/<id>/yeahed" )]
fn user_yeahed(id: i32) -> String {
 	format!( "Requested Yeahed Posts of User {}", id )
}

//Get all posts from user, divided into segments of 20
#[get( "/user/<id>/posts" )]
fn user_posts(id: i32) -> String {
 	format!( "Requested Posts of User {}", id )
}

// Post Functions

//Get posts of community, divided into segments of 20.
#[get( "/posts/<community>/<page>" )]
fn community_posts(community: i32, page: i32) -> String {
 	format!( "Requested Posts of from community {} with page {}", community, page )
}

#[post( "/<community>/<text>/<emotion>" )]
fn community_post_to(community: i32, text: String, emotion: i8 ) -> Status {
	if community == 1 {
		Status::Created
	} else {
		Status::BadRequest
	}
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount( "/", routes![index, user, user_yeahed, user_posts, community_posts, community_post_to, shutdown ] )
}

