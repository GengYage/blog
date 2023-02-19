use yew::prelude::*;
fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <nav>
                <a href="#" class="brand">
                    <span>{"Blog"}</span>
                </a>
                <div class="menu">
                    <a href="#" class="buttun icon-puzzle">{"About"}</a>
                </div>
            </nav>
            <h1>{"hello world!"}</h1>
        </>
    }
}
