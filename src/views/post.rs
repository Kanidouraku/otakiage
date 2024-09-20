use loco_rs::prelude::*;

use crate::models::_entities::posts;

/// Render a list view of posts.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<posts::Model>) -> Result<Response> {
    format::render().view(v, "post/list.html", serde_json::json!({"items": items}))
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
