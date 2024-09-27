use self::runtime::{Runtime, RuntimeContextProvider};
use yew::prelude::*;

mod runtime;

#[function_component]
fn App() -> Html {
    let runtime = use_mut_ref(Runtime::new);

    html! {
        <RuntimeContextProvider context={runtime}>
            <h1>{"Hello, world!"}</h1>
        </RuntimeContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
