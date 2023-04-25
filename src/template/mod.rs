pub mod authenticated;
pub mod public;

pub use askama::Template;
pub use axum::{
    // extract,
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
pub use serde::Deserialize;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate;

// pub async fn base() -> impl IntoResponse {
//     let template = BaseTemplate;
//     HtmlTemplate(template)
// }

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}