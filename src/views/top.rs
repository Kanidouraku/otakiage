use loco_rs::prelude::*;

/// Render a list view of index.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn index(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "index/top.html", serde_json::json!({}))
}
