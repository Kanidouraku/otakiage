use loco_rs::prelude::*;

use crate::models::_entities::posts;

/// Render a list view of auths.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn input(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "auth/input.html", serde_json::json!({}))
}

/// Redirect to the list view of posts.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub async fn add(v: &impl ViewRenderer, items: &Vec<posts::Model>) -> Result<Response> {
    format::render().view(v, "post/list.html", serde_json::json!({"items": items}))
}
