use yew::prelude::*;

#[function_component(LinkList)]
pub fn link_list() -> Html {
    html! {
        <div>
            <h1>{"Link List"}</h1>
            {"Some link"}
        </div>
    }
}
