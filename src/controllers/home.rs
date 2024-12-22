use axum::debug_handler;
use loco_rs::prelude::*;

use crate::views::home::HomeResponse;

#[debug_handler]
async fn current() -> Result<Response> {
    format::html("<h1>Hello World</h1>")
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(current))
}
