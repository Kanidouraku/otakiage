#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::{errors, prelude::*};
use sea_orm::{sea_query::Order, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        self,
        _entities::{
            followers,
            users::{self, ActiveModel, Column, Entity, Model},
        },
    },
    views,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub display_name: String,
    pub user_id: String,
    pub description: String,
    pub followers: Vec<users::Model>,
    pub follower_count: i32,
    pub followings: Vec<users::Model>,
    pub following_count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub user_id: String,
    pub display_name: String,
    pub icon_path: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.user_id = Set(self.user_id.clone());
        item.display_name = Set(self.display_name.clone());
        item.icon_path = Set(self.icon_path.clone());
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
    views::user::list(&v, &item)
}

#[debug_handler]
pub async fn new(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    views::user::create(&v)
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
    views::user::edit(&v, &item)
}

#[debug_handler]
pub async fn show(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    views::user::show(&v, &item)
}

#[debug_handler]
pub async fn add(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    views::user::show(&v, &item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn profile(
    ViewEngine(v): ViewEngine<TeraView>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    println!("profile: {}", id);

    let followers = models::_entities::followers::Entity::find()
        .filter(followers::Column::FolloweeId.eq(id))
        .all(&ctx.db)
        .await?;
    let followings = models::_entities::followers::Entity::find()
        .filter(followers::Column::FollowerId.eq(id))
        .all(&ctx.db)
        .await?;

    let mut follower_users = Vec::new();
    for follower in followers {
        let user = users::Entity::find_by_id(follower.follower_id)
            .one(&ctx.db)
            .await?;
        if let Some(user) = user {
            follower_users.push(user);
        }
    }
    let mut following_users = Vec::new();
    for following in followings {
        let user = users::Entity::find_by_id(following.followee_id)
            .one(&ctx.db)
            .await?;
        if let Some(user) = user {
            following_users.push(user);
        }
    }

    let login_user = load_item(&ctx, id).await?;

    let profile = Profile {
        display_name: login_user.display_name,
        user_id: login_user.user_id,
        description: "近大情報学部 58ハッカソン参加予定".to_string(),
        follower_count: follower_users.len() as i32,
        followers: follower_users,
        following_count: following_users.len() as i32,
        followings: following_users,
    };

    views::user::profile(&v, &profile)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("users/")
        .add("/", get(list))
        .add("/", post(add))
        .add("new", get(new))
        .add(":id", get(show))
        .add(":id/edit", get(edit))
        .add(":id", post(update))
        .add(":id", delete(remove))
        .add(":id/profile", get(profile))
}
