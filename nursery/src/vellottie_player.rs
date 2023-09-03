use lazy_static::lazy_static;
use log::{error, info};
use stylist::yew::styled_component;
use vellottie::runtime::Renderer;
use yew::prelude::*;

lazy_static! {
    static ref RENDERER: Renderer = vellottie::runtime::Renderer::new();
}

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub file: AttrValue,
}

#[styled_component]
pub fn VellottiePlayer(props: &PlayerProps) -> Html {
    let ctr_css = css! {
        display: inline-grid;
        margin: 10px;
    };

    use_effect({
        let path = props.file.to_string();
        move || {
            info!("loading {path}");
            wasm_bindgen_futures::spawn_local(async move {
                let body = reqwest::get(format!("http://127.0.0.1:8080{path}"))
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let lottie =
                    vellottie::import::import_composition(body.as_bytes());
                match lottie {
                    Ok(ref composition) => {
                        info!("Successful read");
                        // todo render
                    }
                    Err(e) => error!("Bad lottie: {e}"),
                }
            });
            // todo load(&path);
        }
    });
    html! {
        <div class={ctr_css}>
            <h1>{"Vellottie"}</h1>
            <div
                style="width:400px;height:400px;background-color:black;"
            />
        </div>
    }
}
