use yew::{function_component, html, use_state, Html, Properties};

use crate::client::game_client::GameClient;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game_id: String,
}

#[function_component(GameResultComponent)]
pub fn game_result_component(props: &Props) -> Html {
    let game_id = &props.game_id;
    let game = use_state(|| None);
    let game_clone = game.clone();
    use_state(move || {
        let game_id = game_id.clone();
        let game_clone = game_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = GameClient::init();
            let game = client.get_game(game_id.clone()).await;
            game_clone.set(game);
        });
    });

    html! {
        <>
            <h1>{format!("Game {}", game_id)}</h1>
            <div>
                <h2>{"Game result"}</h2>
            </div>
        </>
    }
}
