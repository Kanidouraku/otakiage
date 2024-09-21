#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    models::_entities::{
        self,
        users::{ActiveModel, Column},
    },
    views,
};
use axum::debug_handler;
use format::redirect;
use loco_rs::{db::create, prelude::*};
use sea_orm::{sea_query::Order, QueryOrder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub display_name: String,
    pub user_id: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.display_name = Set(self.display_name.clone());
        item.user_id = Set(self.user_id.clone());
    }
}

#[debug_handler]
pub async fn input(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    views::auth::input(&v)
}

#[debug_handler]
pub async fn add(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = ActiveModel {
        icon_path: Set(None),
        ..Default::default()
    };
    params.update(&mut item);
    item.insert(&ctx.db).await?;

    let item = _entities::posts::Entity::find()
        .order_by(Column::Id, Order::Desc)
        .all(&ctx.db)
        .await?;

    views::auth::add(&v, &item).await
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("auths/")
        .add("/", get(input))
        .add("/", post(add))
}
