use yew::prelude::*;
use rustflare::Button;
use rustflare::core::ButtonTrait;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    // Define the `onclick` event handler to increment the counter
    let onclick = {
        let counter = counter.clone();
        move || {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    // Create a new instance of the Button component
    let mut button = Button::new("+1");
    button.set_on_click(onclick);

    html! {
        <div>
            // Render the custom Button component
            {button.render()}
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
// <Button as ButtonTrait<Html>>::render(&button)
