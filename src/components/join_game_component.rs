use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game_id: String,
}

#[function_component(JoinGameComponent)]
pub fn join_game_component(props: &Props) -> Html {
    html! {
        <>
            <section class="bg-sky-950 min-h-screen grid place-items-center flex flex-col">
                <div class="w-full max-w-md w-full mx-auto bg-ct-dark-200 rounded-2xl p-8 space-y-5">
                    <h1 class="text-4xl xl:text-6xl text-center font-[600] text-orange-600 mb-4">
                      {"Game : "}{props.game_id.clone()}
                    </h1>
                </div>
            </section>
        </>
    }
}
