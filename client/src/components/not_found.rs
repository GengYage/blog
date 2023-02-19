use yew::{function_component, html, use_context, Callback, Html};

use crate::components::card::Card;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    use_context::<Callback<String>>()
        .unwrap()
        .emit("Home".into());

    html! {
        <Card title={"404 Not Found!"}>
            <p>{"404 Not Found"}</p>
        </Card>
    }
}
