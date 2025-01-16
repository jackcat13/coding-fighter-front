use std::collections::HashMap;

use yew::{function_component, html, use_state, Callback, Html, Properties};
use yew_router::hooks::use_navigator;

use crate::{
    client::game_client::GameClient, helpers::local_storage::resolve_simple_user_name, Route,
};

const GOLD_TEXT: &str = "text-amber-400";
const SILVER_TEXT: &str = "text-zinc-400";
const BRONZE_TEXT: &str = "text-red-800";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game_id: String,
}

#[function_component(GameResultComponent)]
pub fn game_result_component(props: &Props) -> Html {
    let navigator = use_navigator().expect("Failed to load navigator");
    let game_id = &props.game_id;
    let answers = use_state(Vec::new);
    let answers_clone = answers.clone();
    let on_click = {
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };
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
    let mut results = results
        .iter()
        .map(|(user, score)| Result {
            user: user.clone(),
            score: *score,
        })
        .collect::<Vec<Result>>();
    results.sort_by(|a, b| b.score.cmp(&a.score));
    let mut index = 0;
    let mut previous_score = 0;
    let mut previous_index = 0;
    html! {
        <>
            <section class="bg-sky-950 min-h-screen grid place-items-center flex flex-col">
                <div class="flex flex-col bg-ct-dark-100 rounded-2xl p-8 space-y-5 text-sky-950">
                    <h1>{format!("Game {}", game_id)}</h1>
                    <div>
                        <h2>{"Game result"}</h2>
                        // Display the results
                        <table>
                                { for results.iter().map(|result| {
                                    index += 1;
                                    //TODO : remove next line by fixing backend
                                    let user = result.user.clone().split(":").collect::<Vec<&str>>()[1].to_string().replace("\"", "").replace("}", "");
                                    let user = resolve_simple_user_name(user);
                                    let mut processed_index = index;
                                    if previous_score == result.score {
                                        processed_index = previous_index;
                                    } else {
                                        previous_score = result.score;
                                    }
                                    previous_index = processed_index;
                                    let color = match processed_index {
                                        1 => GOLD_TEXT,
                                        2 => SILVER_TEXT,
                                        3 => BRONZE_TEXT,
                                        _ => ""
                                    };
                                    html! {
                                        <tr><td class={color}>{format!("{}. User: {} - Score: {}", processed_index, user, result.score)}</td></tr>
                                    }
                                })}
                        </table>
                        <button class={"w-full py-3 bg-orange-600 text-white font-semibold rounded-lg outline-none border-none flex justify-center"} onclick={on_click}>{"Home"}</button>
                    </div>
                </div>
            </section>
        </>
    }
}

struct Result {
    user: String,
    score: i8,
}
