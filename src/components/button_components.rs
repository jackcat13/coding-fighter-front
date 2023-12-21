use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct CounterProps {
    pub text: String,
    pub counter: UseStateHandle<u8>,
}

/// The counter button component implementation.
/// It's a button with a counter that appears on the top right of the button.
#[function_component(CounterButton)]
pub fn counter_button_component(props: &CounterProps) -> Html {
    html! {
        <>
            <div class="relative inline-flex w-fit max-w-md w-full">
                <div class="bg-orange-600 absolute bottom-auto left-auto right-0 top-0 z-10 inline-block -translate-y-1/2 translate-x-2/4 rotate-0 skew-x-0 skew-y-0 scale-x-100 scale-y-100 whitespace-nowrap rounded-full px-1.5 py-1 text-center align-baseline text-xs font-bold leading-none text-white">
                    {*props.counter.clone()}
                </div>
                <button
                    type="button"
                    data-te-ripple-color="light"
                    class="max-w-md w-full mb-2 flex rounded px-6 py-2.5 text-xs font-medium uppercase leading-normal text-white shadow-md transition duration-150 ease-in-out hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg justify-center"
                    style="background-color: rgb(62, 185, 145)">
                    {props.text.clone()}
                </button>
            </div>
        </>
    }
}
