use serde::Serialize;
use warp::{Filter, Rejection, Reply};

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

#[allow(opaque_hidden_inferred_bound)]
pub fn hello() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().map(|| "Hello, world!")
}
