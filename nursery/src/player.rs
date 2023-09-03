use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub file: AttrValue,
}

#[styled_component]
pub fn LottiefilesPlayer(props: &PlayerProps) -> Html {
    html! {
        <lottie-player
            src={&props.file}
            background="transparent"
            speed="1"
            style="width: 300px; height: 300px;"
            loop=true
            autoplay=true
        >
        </lottie-player>
    }
}
