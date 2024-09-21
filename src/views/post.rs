use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::{impressions, otakiages, posts};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostWithOtakiageCount {
    pub post: posts::Model,
    pub otakiage_count: i32,
    pub impressions_count: i32,
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
        let impressions = item.find_related(impressions::Entity).one(&ctx.db).await?;
        if let (Some(otakiage), Some(impressions)) = (otakiage, impressions) {
            let otakiage_count = otakiage.count;
            let impressions_count = impressions.count;

            post_with_otakiage_count.push(PostWithOtakiageCount {
                post: item.clone(),
                otakiage_count,
                impressions_count,
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
