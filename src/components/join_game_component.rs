use crate::client::game_client::GameClient;
use crate::dto::game_dto::GameDto;
use crate::helpers::local_storage::local_storage;
use crate::model::game::CURRENT_GAME;
use gloo::utils::window;
use serde_json::to_string;
use yew::{function_component, html, use_state, Html, Properties};

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
    wasm_bindgen_futures::spawn_local(async move {
        gloo::console::log!("Calling the client to get game by id");
        let client = GameClient::init();
        let game_fetched = client.get_game(game_id).await;
        match game_fetched {
            None => {}
            Some(game) => {
                storage
                    .set_item(CURRENT_GAME, to_string(&game.clone()).unwrap().as_str())
                    .expect("Failed to store current game info");
                game_async.set(Some(game));
                gloo::console::log!("Game found !");
            }
        }
    });
    html! {
        <>
            <section class="bg-sky-950 min-h-screen w-full grid place-items-center flex flex-col">
                <div class="w-3/4 mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5 text-sky-950">
                    <h1 class="text-4xl xl:text-6xl text-center font-[600] text-orange-600 mb-4">
                      {"Game : "}
                    </h1>
                    <div>
                        {"Share following URL to invite players : "}
                        {location.href().expect("Failed to resolve host")}
                    </div>
                    {game_info(&game.clone())}
                </div>
            </section>
        </>
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
