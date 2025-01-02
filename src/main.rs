use yew::prelude::*;
mod board;
mod piece;
use board::Board;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <h1>{"Chess Game"}</h1>
            <Board />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}