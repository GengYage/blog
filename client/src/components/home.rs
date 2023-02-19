use yew::{function_component, html, use_context, Callback, Html};

use crate::components::card::Card;

#[function_component(Home)]
pub fn home() -> Html {
    use_context::<Callback<String>>()
        .unwrap()
        .emit("Home".into());

    html! {
        <Card title={"Welcom!"}>
            <p>{"a simple web blog write in rust"}</p>
        </Card>
    }
}
