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
use components::game_result_component::GameResultComponent;
use components::login_component::LoginComponent;
use components::social_media_components::*;
use helpers::local_storage::resolve_user_from_storage;
use helpers::local_storage::USER_SESSION;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

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
    #[at("/game-result/:id")]
    GameResult { id: String },
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

fn game_result(id: String) -> Html {
    html! {
        <>
            {header()}
            <GameResultComponent game_id={id} />
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
    let user_name = resolve_user_from_storage(&local_storage);
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
            Route::GameResult { id } => game_result(id),
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
