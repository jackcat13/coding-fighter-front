use std::collections::HashMap;

use yew::{function_component, html, use_state, Html, Properties};

use crate::client::game_client::GameClient;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game_id: String,
}

#[function_component(GameResultComponent)]
pub fn game_result_component(props: &Props) -> Html {
    let game_id = &props.game_id;
    let answers = use_state(Vec::new);
    let answers_clone = answers.clone();
    use_state(move || {
        let game_id = game_id.clone();
        let game_clone = answers_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = GameClient::init();
            let game_answers = client.get_game_answers(game_id.clone()).await;
            game_clone.set(game_answers);
        });
    });

    let mut results: HashMap<String, i8> = HashMap::new();
    answers.clone().iter().for_each(|answer| {
        if answer.correct_answer == answer.answer {
            if results.contains_key(&answer.user) {
                let current_score = results.get(&answer.user).unwrap();
                results.insert(answer.user.clone(), current_score + 1);
            } else {
                results.insert(answer.user.clone(), 1);
            }
        }
    });
    html! {
        <>
            <h1>{format!("Game {}", game_id)}</h1>
            <div>
                <h2>{"Game result"}</h2>
                // Display the results
                <ul>
                    {for results.iter().map(|(user, score)| {
                        html! {
                            <li>{format!("User: {} - Score: {}", user, score)}</li>
                        }
                    })}
                </ul>
            </div>
        </>
    }
}
