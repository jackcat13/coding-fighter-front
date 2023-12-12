use super::form_input::FormInput;
use super::loading_button_component::LoadingButton;
use crate::helpers::local_storage::local_storage;
use crate::{Route, USER_SESSION};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use validator::Validate;
use web_sys::SubmitEvent;
use yew::{function_component, html, use_state, Callback, Html, NodeRef, UseStateHandle};
use yew_router::prelude::use_navigator;

pub const GUEST_NAME: &str = "guestName";

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    #[validate(length(min = 1, message = "Guest name is required"))]
    guest_name: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        if name == GUEST_NAME {
            data.guest_name = value
        }
        cloned_form.set(data);
    })
}

#[function_component(LoginComponent)]
pub fn login_component() -> Html {
    let navigator = use_navigator().unwrap();
    let form = use_state(LoginUserSchema::default);
    let guest_name_error = use_state(|| "");
    let guest_name_ref = NodeRef::default();
    let handle_guest_name_input = get_input_callback(GUEST_NAME, form.clone());

    let on_submit = {
        let form = form.clone();
        let data = form.deref().clone();
        let guest_name = data.guest_name.clone();
        let guest_name_error = guest_name_error.clone();
        Callback::from(move |event: SubmitEvent| match form.validate() {
            Ok(_) => {
                event.prevent_default();
                let local_storage = local_storage();
                local_storage.set_item(USER_SESSION, &guest_name).unwrap();
                navigator.push(&Route::Home);
            }
            Err(_) => {
                if guest_name.trim().is_empty() {
                    guest_name_error.set("Guest name can't be empty");
                }
            }
        })
    };
    html! {
        <section class="bg-sky-950 min-h-screen grid place-items-center">
            <div class="w-full">
                <h1 class="text-4xl xl:text-6xl text-center font-[600] text-orange-600 mb-4">
                  {"Welcome to the Coding Fighters game."}
                </h1>
                <form onsubmit={on_submit} class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-1 text-slate-500">
                    <FormInput input_type="input" label="Please choose a guest name" name={GUEST_NAME} input_ref={guest_name_ref} handle_onchange={handle_guest_name_input} error={&*guest_name_error} />
                    <LoadingButton
                      loading=false
                      btn_color={Some("bg-orange-600".to_string())}
                      text_color={Some("text-sky-950".to_string())}
                    >
                      {"Login"}
                    </LoadingButton>
                  </form>
            </div>
        </section>
    }
}
