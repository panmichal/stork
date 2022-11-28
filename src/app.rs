use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, PartialEq, Default)]
struct State {
    url: String,
    name: String,
    desc: String,
}

#[derive(Serialize, Deserialize)]
struct SaveArgs<'a> {
    url: &'a str,
    name: &'a str,
    desc: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let name_input_ref = use_ref(|| NodeRef::default());
    let url_input_ref = use_ref(|| NodeRef::default());
    let desc_input_ref = use_ref(|| NodeRef::default());

    let state = use_state(|| State {
        url: String::new(),
        name: String::new(),
        desc: String::new(),
    });
    {
        let state = state.clone();
        let state_dep = state.clone();
        let s = state.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if state.url.is_empty() || state.name.is_empty() || state.desc.is_empty() {
                        return;
                    }

                    let new_msg = invoke(
                        "save_link",
                        to_value(&SaveArgs {
                            url: &*state.url,
                            name: &*state.name,
                            desc: &*state.desc,
                        })
                        .unwrap(),
                    )
                    .await;
                    log(&new_msg.as_string().unwrap());
                });

                || {}
            },
            state_dep,
        );
    }

    let save: Callback<MouseEvent> = {
        let state = state.clone();
        let url_input_ref = url_input_ref.clone();
        let name_input_ref = name_input_ref.clone();
        let desc_input_ref = desc_input_ref.clone();
        Callback::from(move |_| {
            state.set(State {
                url: url_input_ref.cast::<HtmlInputElement>().unwrap().value(),
                name: name_input_ref.cast::<HtmlInputElement>().unwrap().value(),
                desc: desc_input_ref.cast::<HtmlInputElement>().unwrap().value(),
            });
        })
    };

    html! {
        <main class="container">
            // <div class="row">
            //     <a href="https://tauri.app" target="_blank">
            //         <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
            //     </a>
            //     <a href="https://yew.rs" target="_blank">
            //         <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
            //     </a>
            // </div>

            // <p>{"Click on the Tauri and Yew logos to learn more."}</p>
            //<a href="https://www.flaticon.com/free-icons/stork" title="stork icons">Stork icons created by Freepik - Flaticon</a>
            //<a href="https://www.flaticon.com/free-icons/heron" title="heron icons">Heron icons created by edt.im - Flaticon</a>
            // <p>
            //     {"Recommended IDE setup: "}
            //     <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
            //     {" + "}
            //     <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
            //     {" + "}
            //     <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            // </p>

            <div class="row">
                <input id="url-input" ref={&*url_input_ref} placeholder="Enter a url" />
            </div>
            <div class="row">
                <input id="name-input" ref={&*name_input_ref} placeholder="Name" />
            </div>
            <div class="row">
            <input id="desc-input" ref={&*desc_input_ref} placeholder="Description" />

            </div>
            <div class="row"><button class="action-button" type="button" onclick={save}>{"Save"}</button></div>

        </main>
    }
}
