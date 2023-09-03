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
    use_effect({
        let path = props.file.to_string();
        move || {
            load(&path);
        }
    });
    html! {
        <lottie-player
            src={&props.file}
            autoplay=true
            controls=true
            loop=true
            mode="normal"
            style="width: 320px"
        >
        </lottie-player>
    }
}
