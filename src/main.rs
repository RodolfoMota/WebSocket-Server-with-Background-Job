mod api;
mod background;

use actix_web::{App, HttpServer};
use api::hello;
use background::background_job::{background_job};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run the background job
    tokio::spawn(background_job());

    HttpServer::new(|| {
        App::new()
            .service(hello::hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
