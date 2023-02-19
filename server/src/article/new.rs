use std::sync::Arc;

use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse, Responder,
};

use crate::{
    errors::WebError,
    models::{article::Article, user::auth::User},
    AppState,
};

#[web::post("/api/rest/article/add/v1")]
pub async fn add_article(
    user: User,
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, WebError> {
    let user_id = user.id;
    sqlx::query!(
        "insert into articles(title, user_id, content) values ($1, $2, $3)",
        article.title,
        user_id as i64,
        article.content
    )
    .execute(&state.db_pool)
    .await?;

    Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
}
