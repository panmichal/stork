use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub name: String,
    pub desc: String,
}

#[function_component(Link)]
pub fn link(props: &Props) -> Html {
    html! {
        <div>
            <span>{props.url.clone()}</span>
            <span>{props.name.clone()}</span>
            <span>{props.desc.clone()}</span>
        </div>
    }
}
