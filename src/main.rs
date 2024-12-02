mod client;
mod components;
mod dto;
mod helpers;
mod model;

use crate::components::join_game_component::JoinGameComponent;
use crate::components::public_games_component::PublicGamesComponent;
use crate::helpers::local_storage::local_storage;
use components::create_game_component::CreateGameComponent;
use components::game_hub_component::GameHubComponent;
use components::login_component::LoginComponent;
use components::social_media_components::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

const USER_SESSION: &str = "user-session";
const AFTER_UUID_POSITION: usize = 36;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/game")]
    Game,
    #[at("/join-game/:id")]
    JoinGame { id: String },
    #[at("/public-games")]
    PublicGames,
}

fn home() -> Html {
    html! {
        <>
            {header()}
            <GameHubComponent />
            {footer()}
        </>
    }
}

fn game() -> Html {
    html! {
        <>
            {header()}
            <CreateGameComponent />
            {footer()}
        </>
    }
}

fn join_game(id: String) -> Html {
    html! {
        <>
            {header()}
            <JoinGameComponent game_id={id} />
            {footer()}
        </>
    }
}

fn public_games() -> Html {
    html! {
        < >
            { header() }
            < PublicGamesComponent / >
            { footer() }
        < / >
    }
}

fn header() -> Html {
    let local_storage = local_storage();
    let user = local_storage
        .get_item(USER_SESSION)
        .expect("Failed to load user from storage");
    let user_name = &mut String::new();
    match user {
        None => {}
        Some(user) => {
            let _ = &user[AFTER_UUID_POSITION..].clone_into(user_name);
        }
    };
    //TODO proper profile panel
    html! {
        <>
            <div class="absolute top-0 right-0 m-5">
                {"Welcome "}{user_name}
            </div>
        </>
    }
}

fn footer() -> Html {
    html! {
        <>
            <div class="sticky bottom-0 w-full">
                <div class="px-8 place-items-center flex justify-center">
                    <p>{"Join my social networks :"}</p>
                    <DiscordButton />
                    <GithubButton />
                    <LinkedinButton />
                    <YoutubeButton />
                </div>
            </div>
        </>
    }
}

fn not_found() -> Html {
    html! { <h1>{ "404" }</h1> }
}

fn login() -> Html {
    html! {
        <>
            <LoginComponent />
            {footer()}
        </>
    }
}

fn switch(routes: Route) -> Html {
    let local_storage = local_storage();
    let user_session = local_storage.get(USER_SESSION).unwrap();
    match user_session {
        Some(_) => match routes {
            Route::Home => home(),
            Route::NotFound => not_found(),
            Route::Game => game(),
            Route::JoinGame { id } => join_game(id),
            Route::PublicGames => public_games(),
        },
        None => login(),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
