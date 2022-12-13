use std::rc::Rc;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: Rc<str>,
    pub name: Rc<str>,
    pub desc: Rc<str>,
}

#[function_component(Link)]
pub fn link(props: &Props) -> Html {
    let name = if &*props.name == "" {
        props.url.clone()
    } else {
        props.name.clone()
    };

    html! {
        <tr>
        <td>
            <a href={props.url.clone()} target="_blank">{name}</a>
        </td>
        <td>{props.desc.clone()}</td>
        </tr>
    }
}
