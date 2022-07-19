use std::time::{ SystemTime, UNIX_EPOCH };

#[ macro_use ] extern crate rocket;

//I FUCKING LOVE ROCKET
use rocket::{ Shutdown, Data };
use rocket::http::Status;
use rocket::request::{ Request, FromRequest, Outcome };
use rocket_sync_db_pools::{ database, rusqlite };
use rocket::serde::{ Deserialize, json };

use self::rusqlite::params;


use ran::{ set_seeds, Rnum };

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
	text: String,
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
#[get( "/user/<id>/yeahed" )]
fn user_yeahed(id: i64) -> String {
 	format!( "Requested Yeahed Posts of User {}", id )
}

//Get all posts from user, divided into segments of 20
#[get( "/user/<id>/posts" )]
fn user_posts(id: i64) -> String {
 	format!( "Requested Posts of User {}", id )
}

// Post Functions

//Get posts of community, divided into segments of 20.
#[get( "/posts/<community>/<page>" )]
fn community_posts(community: i64, page: i32) -> String {
 	format!( "Requested Posts of from community {} with page {}", community, page )
}

#[post( "/post/<community>/<post>", data= "<img>" )]
async fn community_post_to( db: DB, community: i64, post: String, img: Data<'_> ) -> Status {

	let postdata: NewPost = json::from_str( post.as_str() ).unwrap();

	db.run(move |conn| {

		conn.execute("INSERT INTO posts (id, text, emotion, spoiler, timestamp ) VALUES (?1, ?2, ?3, ?4, ?5)",
				params![ Rnum::newi64().rannum_in( 0.0, 9223372036854775807.0 ).geti64(),
					 postdata.text,
					 postdata.emotion,
					 postdata.spoiler,
					 SystemTime::now().duration_since( UNIX_EPOCH ).unwrap().as_secs()
				])
	}).await.ok();


	if community == 1 {
		Status::Created
	} else {
		Status::BadRequest
	} 
}


#[launch]
fn rocket() -> _ {
	set_seeds( SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos().into() );

	rocket::build().attach( DB::fairing() ).mount( "/", routes![index, user, user_yeahed, user_posts, community_posts, community_post_to, shutdown ] )
}

