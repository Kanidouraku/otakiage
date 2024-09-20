use loco_rs::prelude::*;

use crate::models::_entities::users;

/// Render a list view of users.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<users::Model>) -> Result<Response> {
    format::render().view(v, "user/list.html", serde_json::json!({"items": items}))
}

/// Render a single user view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &users::Model) -> Result<Response> {
    format::render().view(v, "user/show.html", serde_json::json!({"item": item}))
}

/// Render a user create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "user/create.html", serde_json::json!({}))
}

/// Render a user edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &users::Model) -> Result<Response> {
    format::render().view(v, "user/edit.html", serde_json::json!({"item": item}))
}
