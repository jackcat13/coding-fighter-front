mod components;
mod helpers;

use crate::helpers::local_storage::local_storage;
use components::login_component::LoginComponent;
use components::logout_component::LogoutComponent;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

const USER_SESSION: &str = "user-session";

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn home() -> Html {
    let local_storage = local_storage();
    let user = local_storage.get_item(USER_SESSION).unwrap();
    html! {
        <>
            <h1>{"Hello "}{user}</h1>
            <LogoutComponent />
            {footer()}
        </>
    }
}

fn footer() -> Html {
    html! {
        <footer class="footer"><div>

        </div></footer>
    }
}

fn not_found() -> Html {
    html! { <h1>{ "404" }</h1> }
}

fn login() -> Html {
    html! {
        <>
            <LoginComponent />
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
