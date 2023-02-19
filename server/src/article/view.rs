use std::sync::Arc;

use ntex::web::{
    self,
    types::{Json, State},
};

use crate::{
    errors::WebError,
    models::{
        article::{Article, ArticlePreview},
        user::UserPreview,
    },
    AppState,
};

#[web::get("/api/rest/articles/v1")]
pub async fn get_articles(state: State<Arc<AppState>>) -> Result<Json<Vec<Article>>, WebError> {
    let articles = sqlx::query!("select * from articles order by create_time desc")
        .fetch_all(&state.db_pool)
        .await?
        .iter()
        .map(|result| Article {
            id: Some(result.id as u64),
            user_id: result.user_id.map(|a| a as u64),
            title: result.title.clone(),
            content: result.content.clone(),
            create_time: result.create_time,
            update_time: result.update_time,
        })
        .collect::<Vec<Article>>();

    Ok(Json(articles))
}

#[web::get("/api/rest/articles/preview/v1")]
pub async fn get_articles_preview(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<ArticlePreview>>, WebError> {
    let article_previews = sqlx::query!("select a.id,a.user_id, a.title, a.create_time, a.update_time, u.avatar_url,u.name,u.html_url
    from articles a
             left join users u on a.user_id = u.id
    order by a.create_time desc").fetch_all(&state.db_pool).await?.iter().map(|result|{
        ArticlePreview{
            id: Some(result.id as u64),
            user_id: result.user_id.map(|a| a as u64),
            title: result.title.clone(),
            user: UserPreview {
                id:None,
                name: result.name.clone(),
                avatar_url: result.avatar_url.clone(),
                html_url: result.html_url.clone()
            },
            content:None,
            create_time: result.create_time,
            update_time: result.update_time,
        }
    }).collect::<Vec<ArticlePreview>>();

    Ok(Json(article_previews))
}
