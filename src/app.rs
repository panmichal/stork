use crate::components::Navigation;
// use crate::router::switch;
use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
pub mod router;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <BrowserRouter>
            <Navigation />
            <Switch<router::Route> render={router::switch} />
        </BrowserRouter>
        </main>
    }
}
