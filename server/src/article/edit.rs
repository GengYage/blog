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

#[web::post("/api/rest/article/update/v1")]
pub async fn update_article(
    user: User,
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, WebError> {
    let user_id = user.id;

    let id = match article.id {
        Some(id) => id,
        None => return Err(WebError::BadRequest("请传入你要修改的文章id".into())),
    };

    let affected_rows = sqlx::query!(
        "update articles set title = $1 , content = $2 where id = $3 and user_id = $4",
        article.title,
        article.content,
        id as i64,
        user_id as i64,
    )
    .execute(&state.db_pool)
    .await?
    .rows_affected();

    if affected_rows > 0 {
        Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
    } else {
        Err(WebError::BadRequest(format!(
            "article:{id} is not belong to you or is not exits"
        )))
    }
}
