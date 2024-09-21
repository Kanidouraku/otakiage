use loco_rs::prelude::*;

/// Render a list view of auths.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn input(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "auth/input.html", serde_json::json!({}))
}
