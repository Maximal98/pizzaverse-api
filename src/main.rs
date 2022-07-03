#[macro_use] extern crate rocket;

//I FUCKING LOVE ROCKET
use rocket::Shutdown;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite_db")]
struct Db(sqlx::SqlitePool);

#[get("/shutdown")]
fn shutdown( shutdown: Shutdown ) -> Status {
	if 1 == 1 {
		shutdown.notify();
		Status::Ok
	} else {
		Status::Forbidden
	}
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

//Get all posts from user
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

//Post to community, put stuff in headers
#[post( "/posts/<community>" )]
fn community_post_to(community: i32) -> Status {
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
