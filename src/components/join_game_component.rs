use gloo::console::{debug, error};
use gloo::utils::window;
use reqwest_eventsource::Event;
use serde_json::to_string;
use web_sys::SubmitEvent;
use yew::{function_component, html, use_state, Callback, Html, Properties};

use crate::client::game_client::GameClient;
use crate::components::loading_button_component::LoadingButton;
use crate::dto::game_dto::GameDto;
use crate::dto::game_progress_dto::GameProgressDto;
use crate::helpers::local_storage::local_storage;
use crate::model::game::CURRENT_GAME;
use crate::USER_SESSION;
use futures_util::StreamExt;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game_id: String,
}

/// The join game component implementation.
/// It's a page that will call the [GameClient](crate::client::game_client::GameClient) to get a game by id to validate existence of the game.
/// Details of the game will be displayed.
/// Game may be started by the game owner if enough players are connected.
#[function_component(JoinGameComponent)]
pub fn join_game_component(props: &Props) -> Html {
    let location = window().location();
    let storage = local_storage();
    let game = use_state(|| None);
    let game_async = game.clone();
    let game_id = props.game_id.clone();
    let is_start_button = use_state(|| false);
    let is_start_button_clone = is_start_button.clone();
    let on_submit = { Callback::from(move |_event: SubmitEvent| {}) };
    let game_progress = use_state(|| None);
    let game_progress_async = game_progress.clone();
    use_state(move || {
        wasm_bindgen_futures::spawn_local(async move {
            let client = GameClient::init();
            let game_fetched = client.get_game(game_id.clone()).await;
            match game_fetched {
                None => {}
                Some(game) => {
                    let user = storage
                        .get_item(USER_SESSION)
                        .expect("Failed to load current user from local storage");
                    game_async.set(Some(game.clone()));
                    if game.creator == user {
                        let start_button = is_start_button.clone();
                        start_button.set(true);
                    }
                    storage
                        .set_item(CURRENT_GAME, to_string(&game.clone()).unwrap().as_str())
                        .expect("Failed to store current game info");
                }
            }

            let mut es = client.progress_events_souce(game_id);
            while let Some(event) = es.next().await {
                match event {
                    Ok(Event::Open) => debug!("Game progress events open!"),
                    Ok(Event::Message(event)) => {
                        if event.data.as_str() != "NOT STARTED" {
                            let progress: Result<GameProgressDto, _> =
                                serde_json::from_str(event.data.as_str());
                            if let Ok(progress) = progress {
                                game_progress_async.set(Some(progress));
                            }
                        }
                    }
                    Err(_) => {
                        error!("Error during game progress events: {}");
                    }
                }
            }
        });
    });
    let game_progress = game_progress.clone();
    if game_progress.clone().is_some() {
        html! {
            <>
                <section class="bg-sky-950 min-h-screen w-full grid place-items-center flex flex-col">
                    <div class="w-3/4 mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5 text-sky-950">
                        {"Started"}
                    </div>
                </section>
            </>
        }
    } else {
        html! {
            <>
                <section class="bg-sky-950 min-h-screen w-full grid place-items-center flex flex-col">
                    <form onsubmit={on_submit} >
                        <div class="w-3/4 mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5 text-sky-950">
                            <h1 class="text-4xl xl:text-6xl text-center font-[600] text-orange-600 mb-4">
                              {"Game : "}
                            </h1>
                            <div>
                                {"Share following URL to invite players : "}
                                {location.href().expect("Failed to resolve host")}
                            </div>
                            {game_info(&game.clone())}
                            {render_start_button(*is_start_button_clone)}
                        </div>
                    </form>
                </section>
            </>
        }
    }
}

fn game_info(game: &Option<GameDto>) -> Html {
    match game {
        None => {
            html! { <div></div> }
        }
        Some(game) => {
            let game = game.clone();
            html! {
                <>
                    <div>{"Topics : "}{game.topics.join(" - ")}</div>
                    <div>{"Number of questions : "}{game.question_number}</div>
                </>
            }
        }
    }
}

fn render_start_button(is_start_button: bool) -> Html {
    if is_start_button {
        html! {
            <>
                <LoadingButton
                  loading={&false}
                  btn_color={Some("bg-orange-600".to_string())}
                  text_color={Some("text-sky-950".to_string())}
                >
                  {"Start game"}
                </LoadingButton>
            </>
        }
    } else {
        html!()
    }
}
