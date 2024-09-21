use loco_rs::prelude::*;
use sea_orm::SelectColumns;
use serde::{Deserialize, Serialize};

use crate::models::_entities::{otakiages, posts};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostWithOtakiageCount {
    pub post: posts::Model,
    pub otakiage_count: i32,
}

/// Render a list view of posts.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub async fn list(
    v: &impl ViewRenderer,
    items: &Vec<posts::Model>,
    ctx: &AppContext,
) -> Result<Response> {
    let mut post_with_otakiage_count = Vec::new();
    // 各postsのotakiage数をotakiageテーブルからposts_idを使って取得する

    for item in items {
        let otakiage = item.find_related(otakiages::Entity).one(&ctx.db).await?;
        if let Some(otakiage) = otakiage {
            let otakiage_count = otakiage.count;
            post_with_otakiage_count.push(PostWithOtakiageCount {
                post: item.clone(),
                otakiage_count,
            });
        }
    }

    format::render().view(
        v,
        "post/list.html",
        serde_json::json!({"items": post_with_otakiage_count}),
    )
}

/// Render a single post view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &posts::Model) -> Result<Response> {
    format::render().view(v, "post/show.html", serde_json::json!({"item": item}))
}

/// Render a post create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "post/create.html", serde_json::json!({}))
}

/// Render a post edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &posts::Model) -> Result<Response> {
    format::render().view(v, "post/edit.html", serde_json::json!({"item": item}))
}
