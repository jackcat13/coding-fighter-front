use gloo::utils::window;
use serde_json::to_string;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventSource, MessageEvent, SubmitEvent};
use yew::{function_component, html, use_state, Callback, Html, Properties};
use yew_router::hooks::use_navigator;

use crate::client::game_client::GameClient;
use crate::components::loading_button_component::LoadingButton;
use crate::dto::game_dto::GameDto;
use crate::dto::game_progress_dto::GameProgressDto;
use crate::helpers::local_storage::local_storage;
use crate::model::game::CURRENT_GAME;
use crate::{Route, USER_SESSION};

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
    let navigator = use_navigator().expect("Failed to load navigator");
    let location = window().location();
    let storage = local_storage();
    let game = use_state(|| None);
    let game_async = game.clone();
    let game_id_string = props.game_id.clone();
    let game_id_string_async = props.game_id.clone();
    let is_start_button = use_state(|| false);
    let is_start_button_clone = is_start_button.clone();
    let on_submit = {
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let game_id_string_async = game_id_string_async.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = GameClient::init();
                let _ = client.start_game(game_id_string_async.clone()).await;
            });
        })
    };
    let game_id_string_async = game_id_string.clone();
    let on_click = {
        Callback::from(move |_| {
            let game_id_string_async = game_id_string_async.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = GameClient::init();
                client.send_answer(game_id_string_async, 1).await;
            });
        })
    };
    let game_id_string_async = game_id_string.clone();
    let on_click2 = {
        Callback::from(move |_| {
            let game_id_string_async = game_id_string_async.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = GameClient::init();
                client.send_answer(game_id_string_async, 2).await;
            });
        })
    };
    let game_id_string_async = game_id_string.clone();
    let on_click3 = {
        Callback::from(move |_| {
            let game_id_string_async = game_id_string_async.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = GameClient::init();
                client.send_answer(game_id_string_async, 3).await;
            });
        })
    };
    let game_id_string_async = game_id_string.clone();
    let on_click4 = {
        Callback::from(move |_| {
            let game_id_string_async = game_id_string_async.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = GameClient::init();
                client.send_answer(game_id_string_async, 4).await;
            });
        })
    };
    let game_progress = use_state(|| None);
    let game_progress_async = game_progress.clone();
    let game_id_string_async = game_id_string.clone();
    use_state(move || {
        wasm_bindgen_futures::spawn_local(async move {
            let client = GameClient::init();
            let game_fetched = client.get_game(game_id_string.clone()).await;
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

            let cb = Closure::wrap(Box::new(move |event: MessageEvent| {
                let navigator = navigator.clone();
                let game_id_string_async = game_id_string_async.clone();
                if let Some(msg) = event.data().as_string() {
                    if !(msg.eq(&String::from("NOT STARTED")) || msg.eq(&String::from("END"))) {
                        let progress: GameProgressDto = serde_json::from_str(&msg).unwrap();
                        game_progress_async.set(Some(progress));
                    } else if msg.eq(&String::from("END")) {
                        navigator.push(&Route::GameResult {
                            id: game_id_string_async,
                        });
                    }
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            let es = EventSource::new(&client.progress_events_souce_url(&game_id_string)).unwrap();
            es.set_onmessage(Some(cb.as_ref().unchecked_ref()));
            cb.forget();
        });
    });
    if let Some(progress) = &*game_progress.clone() {
        html! {
            <>
                <section class="bg-sky-950 min-h-screen w-full grid place-items-center flex flex-col">
                    <div class="w-3/4 mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5 text-sky-950">
                        <div>{"Questions : "}{&progress.current_question}{" / "}{&progress.question_number}</div>
                        <div>{&progress.question_content.question_text}</div>
                        <div>{"Remaining seconds : "}{&progress.question_content.remaining_time}</div>
                        <div>
                            <button class="w-full py-3 bg-orange-600 text-white font-semibold rounded-lg outline-none border-none flex justify-center" onclick={on_click}>{&progress.question_content.answer_1}</button>
                        </div><div>
                            <button class="w-full py-3 bg-orange-600 text-white font-semibold rounded-lg outline-none border-none flex justify-center" onclick={on_click2}>{&progress.question_content.answer_2}</button>
                        </div><div>
                            <button class="w-full py-3 bg-orange-600 text-white font-semibold rounded-lg outline-none border-none flex justify-center" onclick={on_click3}>{&progress.question_content.answer_3}</button>
                        </div><div>
                            <button class="w-full py-3 bg-orange-600 text-white font-semibold rounded-lg outline-none border-none flex justify-center" onclick={on_click4}>{&progress.question_content.answer_4}</button>
                        </div>
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
