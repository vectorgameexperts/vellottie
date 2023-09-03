use component_container::ComponentContainer;
use log::Level;
use nav::Navigation;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
mod component_container;
mod nav;
mod player;

#[styled_component]
pub fn Root() -> Html {
    let no_margins = css!("html,body {margin: 0;}");
    html! {
        <>
            <Global css={no_margins} />
            <App />
        </>
    }
}

#[styled_component]
pub fn App() -> Html {
    let selected = use_state(|| html!("Nothing selected"));

    let app_css = css! {
        margin: auto;
        max-width: 1100px;
        min-height: 100vh;
    };

    let wrap_css = css! {
        display: flex;
    };

    html! {
        <div id="app" class={app_css}>
            <div id="wrap" class={wrap_css}>
                <Navigation selected={selected.clone()} />
                <ComponentContainer {selected} />
            </div>
        </div>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
    yew::Renderer::<Root>::new().render();
}
