use gloo::{console::log, net::http::Method};
use yew::{
    function_component, html, use_effect_with_deps, use_state, Callback, Html, UseStateHandle,
};

use crate::{fetch::fetch, models::article::ArticlePreview as Preview};

#[function_component(ArticlePreview)]
pub fn article_preview() -> Html {
    let loading = use_state(|| true);
    let articles: UseStateHandle<Result<Vec<Preview>, String>> = use_state(|| Err("".into()));

    {
        let loading = loading.clone();
        let articles = articles.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    articles.set(
                        fetch::<Vec<Preview>>("/api/rest/articles/v1".into(), Method::GET, None)
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
            {content((*articles).clone())}
        }
    }
}

fn content(articles: Result<Vec<Preview>, String>) -> Html {
    let jump = |_: u64| Callback::from(|_| log!("clicked!"));

    match articles {
        Ok(articles) => articles
            .iter()
            .map(|article| {
                html! {
                    <article class = "card" onclick={jump(article.id.unwrap())} key={article.id.unwrap()}>
                        <header>
                            <h3>{&article.title.clone().unwrap()}</h3>
                            <span style="color:grey;">{article.create_time.clone().unwrap()}</span>
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
