use stylist::yew::styled_component;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn load(p: &str);
}

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub file: AttrValue,
}

#[styled_component]
pub fn LottiefilesPlayer(props: &PlayerProps) -> Html {
    let ctr_css = css! {
        display: inline-grid;
        margin: 10px;

        lottie-player {
            border: 1px solid black;
        }
    };

    use_effect({
        let path = props.file.to_string();
        move || {
            load(&path);
        }
    });
    html! {
        <div class={ctr_css}>
            <h1>{"Lottiefiles"}</h1>
            <lottie-player
                src={&props.file}
                autoplay=true
                controls=false
                loop=true
                mode="normal"
                style="width: 400px;height: 400px;"
            >
            </lottie-player>
        </div>
    }
}
