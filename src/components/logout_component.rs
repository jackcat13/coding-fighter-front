use super::loading_button_component::LoadingButton;
use crate::helpers::local_storage::local_storage;
use crate::{Route, USER_SESSION};
use web_sys::SubmitEvent;
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

#[function_component(LogoutComponent)]
pub fn logout_component() -> Html {
    let navigator = use_navigator().unwrap();
    let on_submit = {
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let local_storage = local_storage();
            local_storage.remove_item(USER_SESSION).unwrap();
            navigator.push(&Route::Home);
        })
    };
    html! {
        <form onsubmit={on_submit} class="max-w-md w-full mx-auto overflow-hidden">
            <LoadingButton
              loading=false
              btn_color={Some("bg-orange-600".to_string())}
              text_color={Some("text-sky-950".to_string())}
            >
              {"Logout"}
            </LoadingButton>
        </form>
    }
}
