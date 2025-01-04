use crate::views::{render_form, render_response};
use axum::{
    extract::Form,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NameForm {
    pub name: String,
}

pub async fn get_form() -> impl IntoResponse {
    Html(render_form().into_string())
}

pub async fn handle_submit(Form(form): Form<NameForm>) -> impl IntoResponse {
    Html(render_response(&form.name).into_string())
}
