use nav::Navigation;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
mod nav;
mod player;
mod vellottie_player;

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

        min-height: 100vh;
        width: 100vw;
    };

    let wrap_css = css! {
        display: flex;
    };

    html! {
        <div id="app" class={app_css}>
            <div id="wrap" class={wrap_css}>
                <Navigation selected={selected.clone()} />
                <div style="width: calc(100% - 200px)">
                    <div align="center">
                        {(*selected).clone()}
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();
    yew::Renderer::<Root>::new().render();
}
