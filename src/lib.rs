use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(serde::Serialize)]
struct LoginResponse {
    status: String,
}

fn login_handler(info: web::Json<LoginInfo>) -> impl Responder {
    // Your login logic here
    if info.username == "example_user" && info.password == "example_pass" {
        HttpResponse::Ok().json(LoginResponse { status: "Login successful".to_string() })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse { status: "Login failed".to_string() })
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
