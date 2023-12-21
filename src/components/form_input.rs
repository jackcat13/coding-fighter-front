use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: Option<String>,
    pub label: String,
    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub error: &'static str,
}

/// The form input component implementation.
/// It's a form input with a label and an error message for validation purpose.
#[function_component(FormInput)]
pub fn form_input_component(props: &Props) -> Html {
    let input_type = props
        .input_type
        .clone()
        .unwrap_or_else(|| "text".to_string());

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    html! {
    <div>
      <label html={props.name.clone()} class="block text-sky-950 mb-3">
        {props.label.clone()}
      </label>
      <input
        id={props.name.clone()}
        type={input_type}
        placeholder=""
        class="block w-full rounded-2xl appearance-none focus:outline-none py-2 px-4"
        ref={props.input_ref.clone()}
        onchange={onchange}
      />
    <span class="text-red-500 text-xs pt-1 block">
        {props.error}
    </span>
    </div>
    }
}
