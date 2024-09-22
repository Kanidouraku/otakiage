#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::views;

#[debug_handler]
pub async fn index(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    views::top::index(&v)
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(index))
}
