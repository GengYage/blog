use yew::{function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <article class="card" style="margin:auto;margin-top:5%;width:80%;">
            <header>
                <h3>{&props.title}</h3>
            </header>
            <footer>
                {for props.children.iter()}
            </footer>
        </article>
    }
}