#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{sea_query::Order, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::{
        followers::{self, ActiveModel, Entity, Model},
        users,
    },
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub followee_id: i32,
    pub follower_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.followee_id = Set(self.followee_id);
        item.follower_id = Set(self.follower_id);
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    // TODO: 認証が完成したら現在のユーザーIDでフィルタする(現在は1固定)
    let own_follower_ids = Entity::find()
        .filter(followers::Column::FolloweeId.eq(1))
        .select_only()
        .column(followers::Column::FollowerId)
        .into_tuple::<i32>()
        .all(&ctx.db)
        .await?;

    let followers = users::Entity::find()
        .filter(users::Column::Id.is_in(own_follower_ids))
        .order_by(users::Column::Id, Order::Asc)
        .all(&ctx.db)
        .await?;

    views::follower::list(&v, &followers)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("followers/")
        .add("/", get(list))
        .add(":id", post(update))
}
