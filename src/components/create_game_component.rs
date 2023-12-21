use crate::client::game_client::GameClient;
use crate::components::form_input::FormInput;
use crate::components::loading_button_component::LoadingButton;
use crate::dto::game_dto::GameDto;
use crate::helpers::local_storage::local_storage;
use crate::model::game::*;
use crate::use_navigator;
use crate::Route;
use gloo::utils::document;
use js_sys::Object;
use serde_json::to_string;
use std::ops::Deref;
use wasm_bindgen::__rt::IntoJsResult;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::{function_component, html, use_state, Callback, Html, NodeRef, UseStateHandle};

/// The create game component implementation.
/// It's a form with input fields correspoding to the [Game](crate::model::game::Game) model.
/// When clicking, the form will call the [GameClient](crate::client::game_client::GameClient) to create a game and will redirects user to the [JoinGameComponent](crate::components::join_game_component::JoinGameComponent).
#[function_component(CreateGameComponent)]
pub fn create_game_component() -> Html {
    let storage = local_storage();
    let navigator = use_navigator().expect("Failed to load navigator");
    let form = use_state(GameSchema::default);
    let is_loading = use_state(|| false);
    let game_question_number_error = use_state(|| "");
    let game_question_number_ref = NodeRef::default();
    let handle_game_question_number_input = get_question_number_callback(form.clone());
    let on_submit = {
        let form = form.clone();
        let data = form.deref().clone();
        let is_loading = is_loading.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            is_loading.set(true);
            let navigator = navigator.clone();
            let storage = storage.clone();
            let document = document();
            let topics_node = document.get_elements_by_name(GAME_TOPICS);
            let topics_array = Object::values(&topics_node);
            let topics_html: Vec<HtmlInputElement> = topics_array
                .iter()
                .map(Into::<HtmlInputElement>::into)
                .collect();
            let topics: Vec<String> = topics_html
                .iter()
                .filter(|topic| topic.checked())
                .map(|topic| topic.clone().id())
                .collect();
            let is_private_element = document
                .get_element_by_id(GAME_PRIVATE)
                .expect("Failed to read is game private input");
            let is_private = Into::<HtmlInputElement>::into(
                is_private_element
                    .into_js_result()
                    .expect("Failed to get js value"),
            )
            .checked();
            let client = GameClient::init();
            let game_dto = GameDto {
                id: None,
                topics,
                question_number: data.clone().question_number,
                is_private,
            };
            wasm_bindgen_futures::spawn_local(async move {
                gloo::console::log!("Calling the client to create game");
                let game_created = client.create_game(game_dto.clone()).await;
                storage
                    .set_item(
                        CURRENT_GAME,
                        to_string(&game_created.clone()).unwrap().as_str(),
                    )
                    .expect("Failed to store current game info");
                navigator.push(&Route::JoinGame {
                    id: game_created
                        .id
                        .expect("Failed to resolve id of created game"),
                });
            });
        })
    };
    html! {
        <>
            <section class="bg-sky-950 min-h-screen grid place-items-center flex flex-col">
                <div class="w-full max-w-md w-full mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5">
                    <form onsubmit={on_submit} class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-1 text-slate-500">
                        <fieldset class="grid grid-cols-3">
                            <legend>{"Choose your topics"}</legend>
                            {AVAILABLE_TOPICS.iter().map(|topic| {
                                html!{
                                    <>
                                        <div>
                                            <label for={topic.to_string()}>{topic.to_string()}</label>
                                            <input type="checkbox" class="checkbox mx-4" id={topic.to_string()} name={GAME_TOPICS} />
                                        </div>
                                    </>
                                }
                             }
                            ).collect::<Html>() }
                        </fieldset>
                        <FormInput input_type="number" label="Number of questions" name={GAME_QUESTION_NUMBER} input_ref={game_question_number_ref} handle_onchange={handle_game_question_number_input} error={&*game_question_number_error} />
                        <fieldset>
                            <legend>{"Private ? (share link to join)"}</legend>
                                <div>
                                    <input type="checkbox" class="checkbox mx-4" id={GAME_PRIVATE} name={GAME_PRIVATE} />
                                </div>
                        </fieldset>
                        <LoadingButton
                          loading={&*is_loading}
                          btn_color={Some("bg-orange-600".to_string())}
                          text_color={Some("text-sky-950".to_string())}
                        >
                          {"Create game"}
                        </LoadingButton>
                    </form>
                </div>
            </section>
        </>
    }
}

fn get_question_number_callback(cloned_form: UseStateHandle<GameSchema>) -> Callback<String> {
    Callback::from(move |value: String| {
        let mut data = cloned_form.deref().clone();
        data.question_number = value
            .to_string()
            .parse()
            .expect("Failed to parse question number to int");
        cloned_form.set(data);
    })
}
