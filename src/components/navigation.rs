use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav>
            <div><Link<Route> to={Route::Add}>{ "Add" }</Link<Route>></div>
            <div><Link<Route> to={Route::Show}>{ "Show" }</Link<Route>></div>
        </nav>
    }
}
