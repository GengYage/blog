use std::sync::Arc;

use ntex::{
    util::HashMap,
    web::{
        self,
        types::{Query, State},
        HttpResponse, Responder,
    },
};

use crate::{
    errors::WebError,
    models::user::auth::{Admin, User},
    AppState,
};

#[web::delete("/api/rest/article/delete/v1")]
pub async fn delete_article(
    user: User,
    admin: Option<Admin>,
    state: State<Arc<AppState>>,
    query: Query<HashMap<String, i64>>,
) -> Result<impl Responder, WebError> {
    let user_id = user.id;

    let id = query.get("id").unwrap_or(&0);

    // 非管理员智能删除自己的文章
    let affected_rows = if admin.is_none() {
        sqlx::query!(
            "delete from articles where id = $1 and user_id = $2",
            *id,
            user_id as i64
        )
        .execute(&state.db_pool)
        .await?
    } else {
        sqlx::query!("delete from articles where id = $1", *id)
            .execute(&state.db_pool)
            .await?
    }
    .rows_affected();

    if affected_rows > 0 {
        Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
    } else {
        Err(WebError::BadRequest(format!(
            "article:{id} is not belong to you or is not exits"
        )))
    }
}
