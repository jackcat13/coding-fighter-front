use super::sprinner_component::Spinner;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub loading: bool,
    pub btn_color: Option<String>,
    pub text_color: Option<String>,
    pub children: Children,
}

/// The loading button component implementation.
/// It's a button with a spinner and a text. The spinner is shown when the loading property is true. See [Spinner](crate::components::sprinner_component::Spinner) component.
#[function_component(LoadingButton)]
pub fn loading_button_component(props: &Props) -> Html {
    let text_color = props
        .text_color
        .clone()
        .unwrap_or_else(|| "text-white".to_string());
    let btn_color = props
        .btn_color
        .clone()
        .unwrap_or_else(|| "bg-orange-600".to_string());

    html! {
        <button
          type="submit"
          class={format!(
            "w-full py-3 font-semibold rounded-lg outline-none border-none flex justify-center {}",
             if props.loading {"bg-[#ccc]"} else {btn_color.as_str()}
          )}
        >
          if props.loading {
            <div class="flex items-center gap-3">
              <Spinner />
              <span class="text-slate-500 inline-block">{"Loading..."}</span>
            </div>
          }else{
            <span class={text_color.to_owned()}>{props.children.clone()}</span>
          }
        </button>
    }
}
