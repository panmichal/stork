use crate::components::navigation::Navigation;
use crate::models::link::Link as LinkModel;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::to_value;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Add => html! { <AddForm /> },
        Route::Show => html! { <h1>{ "Home" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">

            //<a href="https://www.flaticon.com/free-icons/stork" title="stork icons">Stork icons created by Freepik - Flaticon</a>
            //<a href="https://www.flaticon.com/free-icons/heron" title="heron icons">Heron icons created by edt.im - Flaticon</a>


            <BrowserRouter>
            <Navigation />
            <Switch<Route> render={switch} />
        </BrowserRouter>
        </main>
    }
}

#[function_component(AddForm)]
fn add_form() -> Html {
    let links_state = use_state(|| vec![]);
    let fields_state = use_state(|| State {
        url: String::new(),
        name: String::new(),
        desc: String::new(),
    });
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

    let cloned_state = fields_state.clone();
    let on_url_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_state.set(State {
            url: value,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = fields_state.clone();
    let on_name_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_state.set(State {
            name: value,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = fields_state.clone();
    let on_desc_change = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_state.set(State {
            desc: value,
            ..cloned_state.deref().clone()
        });
    });

    let save: Callback<MouseEvent> = {
        let state = fields_state.clone();
        let state2 = fields_state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            spawn_local(async move {
                if state.url.is_empty() {
                    return;
                }

                let new_msg = invoke(
                    "save_link",
                    to_value(&SaveArgs {
                        url: &state.url,
                        name: &state.name,
                        desc: &state.desc,
                    })
                    .unwrap(),
                )
                .await;
                log(&new_msg.as_string().unwrap());
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
    <div class="row-right"><button class="action-button" type="button" onclick={save}>{"Save"}</button></div>
    <div class="row">
        // <LinkList links={(*links_state).clone()} />
        </div>
        </>
    }
}
