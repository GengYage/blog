use gloo::net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::Route, fetch::fetch, models::article::ArticlePreview as Preview};

#[function_component(ArticlePreview)]
pub fn article_preview() -> Html {
    let loading = use_state(|| true);
    let articles: UseStateHandle<Result<Vec<Preview>, String>> = use_state(|| Err("".into()));

    // 用于跳转到其他组件
    let navigator = use_navigator().unwrap();

    {
        let loading = loading.clone();
        let articles = articles.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    articles.set(
                        fetch::<Vec<Preview>>(
                            "/api/rest/articles/preview/v1".into(),
                            Method::GET,
                            None,
                        )
                        .await,
                    );
                    loading.set(false)
                })
            },
            (),
        )
    }

    html! {
        if*loading{
            <p>{"Loading ..."}</p>
        } else{
            {content(navigator,(*articles).clone())}
        }
    }
}

fn content(navigator: Navigator, articles: Result<Vec<Preview>, String>) -> Html {
    let jump = |navigator: Navigator, article_id: i64| {
        Callback::from(move |_| navigator.push(&Route::ArticleViewer { article_id }))
    };

    match articles {
        Ok(articles) => articles
            .iter()
            .map(|article| {
                html! {
                    <article class = "card" onclick={jump(navigator.clone(), article.id.unwrap() as i64)} key={article.id.unwrap()}>
                        <header>
                            <h3>{&article.title.clone().unwrap()}</h3>
                            <span style="color:grey;">{article.create_time.clone()}</span>
                            <span style="color:grey;">{article.update_time.clone()}</span>
                            <div>
                                <span style="color:grey;">{article.user.name.clone()}</span>
                                <img src={article.user.avatar_url.clone()}/>
                            </div>
                        </header>
                    </article>
                }
            })
            .collect::<Html>(),
      Err(e) => html! {
            <p>{ e }</p>
    },
    }
}
