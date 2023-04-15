mod api;
mod background;
mod websocket;

use api::hello;
use background::background_job::background_job;
use warp::Filter;
use websocket::my_websocket;

#[tokio::main]
async fn main() {
    // Run the background job
    tokio::spawn(background_job());

    let hello_route = hello::hello();
    let ws_route = my_websocket::websocket();

    let routes = hello_route.or(ws_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
