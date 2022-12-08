use super::FormError;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone)]
struct State {
    url: String,
    name: String,
    desc: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
enum FormState {
    Ready,
    Saving,
}

#[derive(Serialize, Deserialize)]
struct SaveArgs<'a> {
    url: &'a str,
    name: &'a str,
    desc: &'a str,
}

#[function_component(AddForm)]
pub fn add_form() -> Html {
    let fields_state = use_state(|| State {
        url: String::new(),
        name: String::new(),
        desc: String::new(),
    });

    let form_error: UseStateHandle<Option<&str>> = use_state(|| None);

    let cloned_state = fields_state.clone();
    let cloned_form_error = form_error.clone();
    let on_url_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_form_error.set(None);

        cloned_state.set(State {
            url: value,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = fields_state.clone();
    let cloned_form_error = form_error.clone();
    let on_name_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_form_error.set(None);

        cloned_state.set(State {
            name: value,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = fields_state.clone();
    let cloned_form_error = form_error.clone();
    let on_desc_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_form_error.set(None);

        cloned_state.set(State {
            desc: value,
            ..cloned_state.deref().clone()
        });
    });

    let form_error2 = form_error.clone();
    let save: Callback<MouseEvent> = {
        let state = fields_state.clone();
        let state2 = fields_state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            let form_error = form_error2.clone();
            spawn_local(async move {
                if state.url.is_empty() {
                    return;
                }

                let result = invoke(
                    "save_link",
                    to_value(&SaveArgs {
                        url: &state.url,
                        name: &state.name,
                        desc: &state.desc,
                    })
                    .unwrap(),
                )
                .await
                .as_bool();

                if let Some(true) = result {
                    form_error.set(None);
                } else {
                    form_error.set(Some("Error saving link"));
                }
            });

            state2.set(State::default());
        })
    };

    html! {
        <>
        <div class="row">
        <input id="url-input" placeholder="URL" onchange={on_url_change} value={fields_state.deref().clone().url} />
    </div>
    <div class="row">
        <input id="name-input" placeholder="Name" onchange={on_name_change} value={fields_state.deref().clone().name} />
    </div>
    <div class="row">
    <input id="desc-input" placeholder="Description" onchange={on_desc_change} value={fields_state.deref().clone().desc} />

    </div>
    <div class="row-right">
        <FormError error={*form_error.clone()} />
        <button class="action-button" type="button" onclick={save}>{"Save"}</button>
    </div>
        </>
    }
}
