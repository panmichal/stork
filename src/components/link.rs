use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub name: String,
    pub desc: String,
}

#[function_component(Link)]
pub fn link(props: &Props) -> Html {
    let name = if &props.name == "" {
        props.url.clone()
    } else {
        props.name.clone()
    };

    html! {
        <tr>
        <td>
            <a href={props.url.clone()}>{name}</a>
            // <span>{props.desc.clone()}</span>
        </td>
        <td>{props.desc.clone()}</td>
        </tr>
    }
}
