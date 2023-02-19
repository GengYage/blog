use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    article::article_viewer::ArticleViewer, container::Container, home::Home, not_found::NotFound,
};

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <BrowserRouter>
            <Switch<Route> render = {switch}/>
        </BrowserRouter>
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/article/:article_id")]
    ArticleViewer { article_id: i64 },
}

fn switch(route: Route) -> Html {
    html!(
        <Container>
        {
            match route {
                Route::Home =>  html!{<Home/>},
                Route::NotFound => html!{<NotFound/>},
                Route::ArticleViewer{article_id} => html!{<ArticleViewer{article_id}/>}
            }
        }
        </Container>
    )
}
