use chrono::Timelike;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::{impressions, otakiages, posts, users};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostWithOtakiageCount {
    pub post: posts::Model,
    pub otakiage_count: i32,
    pub impressions_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListViewPost {
    pub display_name: String,
    pub user_id: String,
    pub elapsed_time_since_post: u32,
    pub content: String,
    pub otakiage_id: i32,
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
    let mut recommended_posts = Vec::new();
    // 各postsのotakiage数をotakiageテーブルからposts_idを使って取得する

    for item in items {
        let otakiage = item.find_related(otakiages::Entity).one(&ctx.db).await?;
        let impressions = item.find_related(impressions::Entity).one(&ctx.db).await?;
        if let (Some(otakiage), Some(impressions)) = (otakiage, impressions) {
            let otakiage_count = otakiage.count;
            let impressions_count = impressions.count;

            let user = item.find_related(users::Entity).one(&ctx.db).await?;
            let elapsed_time_since_post_second =
                chrono::Utc::now().timestamp() - item.created_at.timestamp();
            let elapsed_time_since_post = elapsed_time_since_post_second as u32 / 60;

            if let Some(user) = user {
                recommended_posts.push(ListViewPost {
                    display_name: user.display_name.clone(),
                    user_id: user.user_id.clone(),
                    elapsed_time_since_post,
                    content: item.content.clone().unwrap_or("Empty".to_string()),
                    otakiage_id: otakiage.id,
                    otakiage_count,
                    impressions_count,
                });
            }
        }
    }

    format::render().view(
        v,
        "post/list.html",
        serde_json::json!({"recommended_posts": recommended_posts, "followed_posts": recommended_posts}),
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
