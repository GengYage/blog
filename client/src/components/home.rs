use yew::{function_component, html, use_context, Callback, Html};

use crate::components::{article::articles_preview::ArticlePreview, card::Card};

#[function_component(Home)]
pub fn home() -> Html {
    use_context::<Callback<String>>()
        .unwrap()
        .emit("Home".into());

    html! {
        <Card title={"Articles"}>
            <ArticlePreview/>
        </Card>
    }
}
