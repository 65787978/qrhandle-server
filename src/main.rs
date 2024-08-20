/*
Use:

curl GET http://localhost:3000/

curl -X POST http://localhost:3000/users -H "Content-Type: application/json" -d "{\"username\":\"YourUsername\", \"email\":\"
YourEmail@email.com\"}"

*/
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

static mut USER_ID: u64 = 0;

#[tokio::main]
async fn main() {
    /* Initialize tracing */
    tracing_subscriber::fmt::init();

    /* Build our application with a route */
    let app = Router::new()
        /* GET request of '/' */
        .route("/", get(root))
        /* POST request of '/users' */
        .route("/users", post(create_user));

    /* Run our app with hyper, listening locally on port 3000, for global listening use 0.0.0.0:3000 */
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    /* Serve the app */
    axum::serve(listener, app).await.unwrap();
}

/* GET / */
async fn root() -> &'static str {
    "Hello, World!"
}

/* POST /users */
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    /* App logic here */
    unsafe { USER_ID += 1 };
    let user = User {
        id: unsafe { USER_ID },
        username: payload.username,
        email: payload.email,
    };

    /* Convert to JSON response with status code '201 Created' */
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    email: String,
}
