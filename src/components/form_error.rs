use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub error: Option<&'static str>,
}

#[function_component(FormError)]
pub fn form_error(props: &Props) -> Html {
    html! {
        <span class="form-error">
            { props.error.unwrap_or("") }
            </span>
    }
}
