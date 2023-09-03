use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ComponentContainerProps {
    pub selected: UseStateHandle<Html>,
}

#[styled_component]
pub fn ComponentContainer(props: &ComponentContainerProps) -> Html {
    let container_css = css! {
        background-color: #575757;
        width: 100%;
    };

    html! {
        <div align="center" class={container_css}>
            {(*props.selected).clone()}
        </div>
    }
}
