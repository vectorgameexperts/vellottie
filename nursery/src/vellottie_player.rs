use gloo_timers::callback::Interval;
use lazy_static::lazy_static;
use log::{error, info};
use std::sync::{Arc, Mutex};
use stylist::yew::styled_component;
use vello::{
    kurbo::Affine, peniko::Color, util::RenderContext, RenderParams, Scene,
    SceneBuilder,
};
use vellottie::runtime::{
    vello::{self, util::RenderSurface, RendererOptions},
    Composition,
};
use winit::{
    dpi::LogicalSize, event_loop::EventLoopBuilder,
    platform::web::WindowExtWebSys, window::WindowBuilder,
};
use yew::prelude::*;

lazy_static! {
    static ref RENDER_STATE: Arc<Mutex<Option<RenderState>>> =
        Arc::new(Mutex::new(None));
    static ref COMPOSITION: Arc<Mutex<Option<Composition>>> =
        Arc::new(Mutex::new(None));
    static ref ANIM_START: instant::Instant = instant::Instant::now();
}

struct RenderState {
    ctx: RenderContext,
    surface: RenderSurface,
    vellottie_renderer: vellottie::runtime::Renderer,
    vello_renderer: vellottie::runtime::vello::Renderer,
}

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub file: AttrValue,
}

#[styled_component]
pub fn VellottiePlayer(props: &PlayerProps) -> Html {
    let baseurl = web_sys::window().unwrap().origin();

    let ctr_css = css! {
        display: inline-grid;
        margin: 10px;

        canvas {
            border: 1px solid black;
        }
    };

    use_effect({
        let path = props.file.to_string();
        move || {
            // Initialize render loop
            let interval = Interval::new(2, move || {
                if let Some(composition) =
                    (*COMPOSITION).lock().unwrap().as_ref()
                {
                    let time = (*ANIM_START).elapsed().as_secs_f32();
                    render(composition, time);
                }
            });

            wasm_bindgen_futures::spawn_local(async move {
                // Init GPU Canvas, if not initialized.
                init_state().await;
                // Load file
                info!("loading {path}...");
                let body = reqwest::get(format!("{baseurl}{path}"))
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                info!("retrieved contents, parsing...");
                let lottie =
                    vellottie::import::import_composition(body.as_bytes());
                match lottie {
                    Ok(ref composition) => {
                        info!("read file successfully");
                        (*COMPOSITION)
                            .lock()
                            .unwrap()
                            .replace(composition.to_owned());
                    }
                    Err(e) => error!("Bad lottie: {e}"),
                }
            });

            || {
                interval.cancel(); // cleanup
            }
        }
    });
    html! {
        <div id="canvas_holster" class={ctr_css}>
            <h1>{"Vellottie"}</h1>
        </div>
    }
}

async fn init_state() {
    if (*RENDER_STATE).lock().unwrap().is_some() {
        return;
    }
    // Create the GPU Canvas
    info!("creating GPU canvas...");
    let event_loop = EventLoopBuilder::new().build();
    #[allow(unused_mut)]
    let mut ctx = RenderContext::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(400, 400))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    // On wasm, append the canvas to the document body
    let canvas = window.canvas();
    let size = window.inner_size();
    canvas.set_width(size.width);
    canvas.set_height(size.height);
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("canvas_holster"))
        .and_then(|parent| parent.append_child(canvas.as_ref()).ok())
        .expect("couldn't append canvas to document");
    _ = web_sys::HtmlElement::from(canvas).focus();

    let size = window.inner_size();
    let surface = ctx.create_surface(&window, size.width, size.height).await;
    let device_handle = &ctx.devices[surface.dev_id];
    let vellottie_renderer = vellottie::runtime::Renderer::new();
    let vello_renderer = vellottie::runtime::vello::Renderer::new(
        &device_handle.device,
        &RendererOptions {
            surface_format: Some(surface.format),
        },
    )
    .unwrap();
    (*RENDER_STATE).lock().unwrap().replace(RenderState {
        vellottie_renderer,
        vello_renderer,
        ctx,
        surface,
    });
    info!("GPU canvas created");
}

fn render(composition: &Composition, time: f32) {
    let mut state_lock = (*RENDER_STATE).lock().unwrap();
    let state: &mut RenderState = state_lock.as_mut().unwrap();
    let device_handle = &state.ctx.devices[state.surface.dev_id];
    let mut scene = Scene::new();

    let width = state.surface.config.width;
    let height = state.surface.config.height;
    let transform = Affine::scale(1.0);

    let mut builder = SceneBuilder::for_scene(&mut scene);
    state.vellottie_renderer.render(
        composition,
        time,
        transform,
        1.0,
        &mut builder,
    );

    let surface_texture = state
        .surface
        .surface
        .get_current_texture()
        .expect("failed to get surface texture");
    state
        .vello_renderer
        .render_to_surface(
            &device_handle.device,
            &device_handle.queue,
            &scene,
            &surface_texture,
            &RenderParams {
                base_color: Color::BLACK,
                width,
                height,
            },
        )
        .expect("failed to render to surface");
    surface_texture.present();
}
