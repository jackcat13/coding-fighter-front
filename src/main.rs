use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn home() -> Html {
    html! {
        <>
            <h1>{"TODO"}</h1>
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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::NotFound => not_found(),
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
