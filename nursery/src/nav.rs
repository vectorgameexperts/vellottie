use crate::player;
use crate::vellottie_player;
use include_dir::{include_dir, Dir, DirEntry};
use stylist::yew::styled_component;
use yew::prelude::*;

static FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/files");

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub selected: UseStateHandle<Html>,
}

#[styled_component]
pub fn Navigation(props: &NavigationProps) -> Html {
    let mut items = vec![];
    for file in FILES.entries().iter().filter_map(DirEntry::as_file) {
        let path = file.path();
        let mut name = path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        name.truncate(name.len() - 5); // .json
        log::info!("file: {}", path.to_string_lossy());
        items.push((
            name,
            html!{
                <>
                    <player::LottiefilesPlayer file={format!("/files/{}", path.display())} />
                    <vellottie_player::VellottiePlayer file={format!("/files/{}", path.display())} />
                </>
            },
        ));
    }
    items.sort_by(|i1, i2| i1.0.cmp(&i2.0));

    let change_selected = {
        let selected = props.selected.clone();
        move |new_selection: Html| {
            selected.set(new_selection);
        }
    };

    let nav_css = css! {
        margin: 0;
        background-color: #dddddd;
        display: flex;
        flex-direction: column;
        min-height: 100vh;
        width: 200px;
        overflow-y: auto;
    };

    html! {
        <div class={nav_css}>
            {
                items.into_iter().map(|(name, item)| {
                    let change_selected = change_selected.clone();

                    html!{
                        <div>
                            <input
                                type="radio"
                                name={"selection"}
                                onchange={Callback::from(
                                    move |_e| {
                                        change_selected(item.clone());
                                    }
                                )}
                            />
                            <label>
                                {name}
                            </label>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
