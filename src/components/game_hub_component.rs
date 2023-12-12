use crate::components::button_components::CounterButton;
use crate::components::logout_component::LogoutComponent;
use crate::Route;
use yew::{function_component, html, use_state, Callback, Html};
use yew_router::prelude::{use_navigator, Navigator};

#[function_component(GameHubComponent)]
pub fn game_hub_component() -> Html {
    let navigator = use_navigator().expect("Failed to load navigator");
    let game_count = use_state(|| 0);
    html! {
        <>
            <section class="bg-sky-950 min-h-screen grid place-items-center flex flex-col">
                <div class="w-full max-w-md w-full mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5">
                    <CounterButton text="Join a game" counter={game_count} />
                    {create_game_button(navigator.clone())}
                    <LogoutComponent />
                </div>
            </section>
        </>
    }
}
fn create_game_button(navigator: Navigator) -> Html {
    let on_click = {
        Callback::from(move |_| {
            navigator.push(&Route::Game);
        })
    };
    html! {
        <>
            <button
                onclick={on_click}
                type="button"
                class="max-w-md w-full inline-block rounded bg-success px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-white shadow-[0_4px_9px_-4px_#14a44d] transition duration-150 ease-in-out hover:bg-success-600 hover:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.3),0_4px_18px_0_rgba(20,164,77,0.2)] focus:bg-success-600 focus:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.3),0_4px_18px_0_rgba(20,164,77,0.2)] focus:outline-none focus:ring-0 active:bg-success-700 active:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.3),0_4px_18px_0_rgba(20,164,77,0.2)] dark:shadow-[0_4px_9px_-4px_rgba(20,164,77,0.5)] dark:hover:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.2),0_4px_18px_0_rgba(20,164,77,0.1)] dark:focus:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.2),0_4px_18px_0_rgba(20,164,77,0.1)] dark:active:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.2),0_4px_18px_0_rgba(20,164,77,0.1)]"
                style="background-color: rgb(62, 185, 145)">
                    { "Create a game" }
            </button>
        </>
    }
}
