#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::views;

#[debug_handler]
pub async fn input(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    views::auth::input(&v)
}

pub fn routes() -> Routes {
    Routes::new().prefix("auths").add("/", get(input))
}
