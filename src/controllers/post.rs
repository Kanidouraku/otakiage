#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{sea_query::Order, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        self,
        _entities::posts::{ActiveModel, Column, Entity, Model},
    },
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub content: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.content = Set(self.content.clone());
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
    let item = Entity::find()
        .order_by(Column::Id, Order::Desc)
        .all(&ctx.db)
        .await?;

    let posts = Entity::find().all(&ctx.db).await?;
    // すべてのpostのimpressions数を増加
    for post in posts {
        let impressions = post
            .find_related(models::_entities::impressions::Entity)
            .one(&ctx.db)
            .await?;
        if let Some(mut impressions) = impressions {
            impressions.count += 1;
            let count = impressions.count;
            let mut active_impressions = impressions.into_active_model();
            active_impressions.count = Set(count);
            active_impressions.update(&ctx.db).await?;
        }
    }

    views::post::list(&v, &item, &ctx).await
}

#[debug_handler]
pub async fn new(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    views::post::create(&v)
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

#[debug_handler]
pub async fn edit(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    views::post::edit(&v, &item)
}

#[debug_handler]
pub async fn show(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    views::post::show(&v, &item)
}

#[debug_handler]
pub async fn add(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = ActiveModel {
        user_id: Set(1),
        ..Default::default()
    };
    params.update(&mut item);

    let item = item.insert(&ctx.db).await?;
    models::_entities::otakiages::ActiveModel {
        post_id: Set(item.id),
        count: Set(0),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;
    models::_entities::impressions::ActiveModel {
        post_id: Set(item.id),
        count: Set(0),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;
    views::post::show(&v, &item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("posts/")
        .add("/", get(list))
        .add("/", post(add))
        .add("new", get(new))
        .add(":id", get(show))
        .add(":id/edit", get(edit))
        .add(":id", post(update))
        .add(":id", delete(remove))
}
