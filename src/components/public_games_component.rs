use crate::client::game_client;
use crate::dto::game_dto::GameDto;
use crate::Route;
use std::rc::Rc;
use yew::html::onclick::Event;
use yew::{function_component, html, use_state, Callback, Html, UseStateHandle};
use yew_router::prelude::{use_navigator, Navigator};

/// The public games component implementation.
/// It's a page with a list of public games which can be joined using the join button.
/// Join button redirects user to the [JoinGameComponent](crate::components::join_game_component::JoinGameComponent).
/// Games are fetched from the server using the [GameClient](crate::client::game_client::GameClient).
/// If there are no games, the page will be empty.
#[function_component(PublicGamesComponent)]
pub fn public_games_component() -> Html {
    let navigator = use_navigator().expect("Failed to load navigator");
    let games: UseStateHandle<Vec<GameDto>> = use_state(Vec::new);
    let games_clone = games.clone();
    use_state(move || {
        wasm_bindgen_futures::spawn_local(async move {
            games_clone.set(game_client::GameClient::init().get_games().await);
        });
    });
    html! {
        <>
            <section class="bg-sky-950 min-h-screen grid place-items-center flex flex-col">
                <div class="w-full max-w-md w-full mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5">
                    {
                        games.clone().iter()
                            .map(|game| game_html(game, &navigator))
                            .collect::<Html>()
                    }
                </div>
            </section>
        </>
    }
}

fn game_html(game: &GameDto, navigator: &Navigator) -> Html {
    let game = game.clone();
    let navigator = navigator.clone();
    let game_rc = Rc::new(game.clone());
    let on_click: Callback<Event> = Callback::from(move |_| {
        navigator.push(&Route::JoinGame {
            id: game_rc.id.clone().expect("Failed to resolve game id"),
        })
    });
    html! {
        <div class="flex flex-col bg-ct-dark-100 rounded-2xl p-8 space-y-5 text-sky-950">
            <div>{"Game : "}{game.id.expect("Failed to resolve game id")}</div>
            <div>{"Topics : "}{game.topics.join(" - ")}</div>
            <div>{"Number of questions : "}{game.question_number}</div>
            <div>
                <button
                    onclick={on_click}
                    type="button"
                    class="max-w-md w-full inline-block rounded bg-success px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-white shadow-[0_4px_9px_-4px_#14a44d] transition duration-150 ease-in-out hover:bg-success-600 hover:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.3),0_4px_18px_0_rgba(20,164,77,0.2)] focus:bg-success-600 focus:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.3),0_4px_18px_0_rgba(20,164,77,0.2)] focus:outline-none focus:ring-0 active:bg-success-700 active:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.3),0_4px_18px_0_rgba(20,164,77,0.2)] dark:shadow-[0_4px_9px_-4px_rgba(20,164,77,0.5)] dark:hover:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.2),0_4px_18px_0_rgba(20,164,77,0.1)] dark:focus:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.2),0_4px_18px_0_rgba(20,164,77,0.1)] dark:active:shadow-[0_8px_9px_-4px_rgba(20,164,77,0.2),0_4px_18px_0_rgba(20,164,77,0.1)]"
                    style="background-color: rgb(62, 185, 145)">
                        { "Join" }
                </button>
            </div>
        </div>
    }
}
