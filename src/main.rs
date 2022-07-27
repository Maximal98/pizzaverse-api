use std::time::SystemTime;

#[ macro_use ] extern crate rocket;

//I FUCKING LOVE ROCKET
use rocket::{ Shutdown, Data };
use rocket::http::Status;
use rocket::request::{ Request, FromRequest, Outcome };
use rocket_sync_db_pools::{ database, rusqlite };
use rocket::serde::{ Deserialize, json, json::Json };

use self::rusqlite::params;

#[database("db")]
struct DB(rusqlite::Connection);

struct SUAuth<'r>(&'r str);

#[ derive( Debug ) ]
enum KeyError {
	Missing,
	Invalid,
}

#[ derive( Deserialize )]
#[ serde( crate = "rocket::serde" ) ]
struct NewPost {
	content: String,
	emotion: i8,
	spoiler: bool
}

#[ rocket::async_trait ]
impl<'r> FromRequest<'r> for SUAuth<'r> {
	type Error = KeyError;

	async fn from_request( req: &'r Request<'_> ) -> Outcome<Self, Self::Error> {
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
fn user(id: i64) -> Status {
	
	if id != 69 {
		Status::NotFound
	} else {
		Status::Ok
	}
}

//Get all yeahed posts from user
#[get( "/user/<id>/yeahed/<page>" )]
fn user_yeahed( id: String, page: i32 ) -> String {
 	format!( "Requested Yeahed Posts of User {} at page {}", id, page )
}

//Get all posts from user, divided into segments of 20
#[get( "/user/<id>/posts/<page>" )]
fn user_posts( id: String, page: i32 ) -> String {
 	format!( "Requested Posts of User {} at page {}", id, page )
}

// Post Functions

//Get posts of community, divided into segments of 20.
#[get( "/posts/<community>/<page>" )]
fn community_posts(community: String, page: i32) -> String {
 	format!( "Requested Posts of from community {} with page {}", community, page )
}

#[post( "/post/<community>", format = "json", data= "<post>" )]
async fn community_post_to( db: DB, community: String, post: Json<NewPost> ) -> Status {

	let community_clone = community.clone();

	db.run(move |conn| {

		
		conn.execute("INSERT INTO posts ( id, community, poster, content, emotion, spoiler, timestamp ) VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7 )",
				params![ random_string::generate( 16, "abcdefghijklmnopqrstuvwxyz1234567890" ),
					 community_clone,
					 "notices your placeholder text UwU",
					 post.content,
					 post.emotion,
					 post.spoiler,
					 SystemTime::now().duration_since( SystemTime::UNIX_EPOCH ).unwrap().as_secs()
				       ]
			    )
	}).await.ok();


	if community == "SEX" {
		Status::Created
	} else {
		Status::BadRequest
	} 
}


#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach( DB::fairing() )
		.mount( "/", routes![index,
				     user,
				     user_yeahed,
				     user_posts,
				     community_posts,
				     community_post_to,
				     shutdown 
				    ]
		      )
}

