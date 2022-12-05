use super::link::Link;
use crate::models::link::Link as LinkModel;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub links: Vec<LinkModel>,
}

#[function_component(LinkList)]
pub fn link_list(props: &Props) -> Html {
    html! {
        <div>
            <h1>{"Link List"}</h1>
            <ol class={"link-list"}>
                { for props.links.iter().map(|link| {
                    html! {
                        <li>
                            <Link url={link.url.clone()} name={link.name.clone()} desc={link.desc.clone()}/>
                        </li>
                    }
                })}
                </ol>
        </div>
    }
}
