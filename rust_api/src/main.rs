use warp::Filter;
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;

#[derive(Deserialize)]
struct GoogleAuthRequest {
    google_id: String,
    name: String,
    email: String,
    picture_url: Option<String>,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

#[derive(Serialize)]
struct User {
    id: i32,
    google_id: String,
    name: String,
    email: String,
    picture_url: Option<String>,
}

fn get_db_connection() -> Result<PooledConn> {
    let url = "mysql://root:2024@localhost:3306/dbauth";
    let pool = Pool::new(url)?;
    pool.get_conn()
}

async fn get_users() -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = get_db_connection().map_err(|_| warp::reject::reject())?;
    
    let users: Vec<(i32, String, String, String, Option<String>)> = conn.exec(
        "SELECT id, google_id, name, email, picture_url FROM users",
        (),
    ).map_err(|_| warp::reject::reject())?;

    let user_list: Vec<User> = users.into_iter().map(|(id, google_id, name, email, picture_url)| {
        User { id, google_id, name, email, picture_url }
    }).collect();

    Ok(warp::reply::json(&user_list))
}

async fn google_auth(req: GoogleAuthRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = get_db_connection().map_err(|_| warp::reject::reject())?;
    
    let existing_user: Option<(i32,)> = conn.exec_first(
        "SELECT id FROM users WHERE google_id = ?",
        (&req.google_id,),
    ).map_err(|_| warp::reject::reject())?;

    match existing_user {
        Some(_) => {
            Ok(warp::reply::json(&ApiResponse {
                message: "Google login successful".to_string(),
            }))
        }
        None => {
            conn.exec_drop(
                "INSERT INTO users (google_id, name, email, picture_url) VALUES (?, ?, ?, ?)",
                (&req.google_id, &req.name, &req.email, &req.picture_url),
            ).map_err(|_| warp::reject::reject())?;
            
            Ok(warp::reply::json(&ApiResponse {
                message: "Google user registered and logged in".to_string(),
            }))
        }
    }
}

async fn google_callback() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::html("<h1>OAuth Callback Received</h1><script>window.close();</script>"))
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_credentials(true);

    let get_users_route = warp::path("users")
        .and(warp::get())
        .and_then(get_users);

    let google_auth_route = warp::path("google-auth")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(google_auth);

    let google_callback_route = warp::path!("auth" / "google" / "callback")
        .and(warp::get())
        .and_then(google_callback);

    let routes = get_users_route.or(google_auth_route).or(google_callback_route).with(cors);

    println!("Server running on http://localhost:8081");
    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}