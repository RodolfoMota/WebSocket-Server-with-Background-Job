mod api;
mod background;

use actix_web::{App, HttpServer};
use api::auth;
use background::cleanSession::{clean_sessions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run the background job
    tokio::spawn(clean_sessions());

    HttpServer::new(|| {
        App::new()
            .service(auth::hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
