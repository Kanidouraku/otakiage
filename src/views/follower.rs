use loco_rs::prelude::*;

use crate::models::_entities::users;

/// Render a list view of followers.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, followers: &Vec<users::Model>) -> Result<Response> {
    format::render().view(
        v,
        "follower/list.html",
        serde_json::json!({"followers": followers}),
    )
}
