use super::link::Link;
use crate::models::link::Link as LinkModel;
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub links: Vec<LinkModel>,
}

#[function_component(LinkList)]
pub fn link_list() -> Html {
    let links_state = use_state(|| vec![]);
    {
        let links_state = links_state.clone();
        use_effect(move || {
            spawn_local(async move {
                let new_msg = invoke("get_links", to_value(&()).unwrap()).await;
                let links: Vec<LinkModel> = from_value(new_msg).unwrap();
                links_state.set(links);
            });
            || {}
        });
    }

    html! {
        <div>
            <table class={"link-list"}>
                { for links_state.iter().map(|link| {
                    html! {
                        <Link url={link.url.clone()} name={link.name.clone()} desc={link.desc.clone()}/>
                    }
                })}
                </table>
        </div>
    }
}
