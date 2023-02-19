use yew::{function_component, html, Callback, Children, ContextProvider, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let set_title = Callback::from(move |content: String| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&format!("{content} - Blog"));
    });

    let jump = move |route: Route| Callback::from(move |_| navigator.push(&route));

    html! {
       <>
        <nav>
            <a onclick={jump(Route::Home)} class="brand">
                <span>{ "Blog" }</span>
            </a>
            // <div class="name">
            //  <a href="#" class="button icon-puzzle">{"About"}</a>
            // </div>
        </nav>
        <ContextProvider<Callback<String>> context = {set_title}>
            {
                for props.children.iter()
            }
        </ContextProvider<Callback<String>>>
       </>
    }
}
