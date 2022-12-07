use crate::components::{AddForm, LinkList};
use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Add,
    #[at("/show")]
    Show,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Add => html! { <AddForm /> },
        Route::Show => html! { <LinkList /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
