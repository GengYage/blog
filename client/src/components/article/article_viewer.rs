use gloo::net::http::Method;
use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

use crate::{components::card::Card, fetch, models::article::Article};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub article_id: i64,
}

#[function_component(ArticleViewer)]
pub fn article_viewer(props: &Props) -> Html {
    let loading = use_state(|| true);
    let article: UseStateHandle<Result<Article, String>> = use_state(|| Err("".into()));

    let article_id = props.article_id;
    {
        let loading = loading.clone();
        let article = article.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    article.set(
                        fetch::fetch::<Article>(
                            format!("/api/rest/article/get/v1?id={}", article_id.clone()),
                            Method::GET,
                            None,
                        )
                        .await,
                    );
                    loading.set(false)
                });
            },
            (),
        );
    }

    let title = if let Ok(article) = (*article).clone() {
        article.title.clone().unwrap()
    } else {
        "文章".into()
    };

    use_context::<Callback<String>>()
        .unwrap()
        .emit(title.clone());

    html! {
        if * loading {
            <Card title={"Loading.."}>
                <p>{"正在准备数据,马上就好"}</p>
            </Card>
        } else {
            <Card {title}>
                {
                    match &*article {
                        Ok(article) => convert_markdown_to_html(article),
                        Err(e) => html!(<p>{e}</p>),
                    }
                }
            </Card>
        }
    }
}

fn convert_markdown_to_html(article: &Article) -> Html {
    let content = &article.content.clone().unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);

    let mut markdown_html = String::new();
    html::push_html(&mut markdown_html, parser);

    let div_wrapper = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    // 把解析的 HTML 放进去
    div_wrapper.set_inner_html(&markdown_html);
    let node: web_sys::Node = div_wrapper.into();

    Html::VRef(node)
}
