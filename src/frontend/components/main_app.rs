use yew::prelude::*;
use crate::components::sidebar::Sidebar;
use crate::components::post_explorer::PostExplorer;
use crate::components::media::MediaView;
use crate::components::settings::SettingsView;

#[function_component(MainApp)]
pub fn main_app() -> Html {
    let selected_view = use_state(|| "post".to_string());

    let on_sidebar_select = {
        let selected_view = selected_view.clone();
        Callback::from(move |section: String| selected_view.set(section))
    };

    html! {
        <div class="main-app">
            <Sidebar on_select={on_sidebar_select} />
            <div class="main-content">
                { match selected_view.as_str() {
                    "post" => html! { <PostExplorer /> },
                    "media" => html! { <MediaView /> },
                    "settings" => html! { <SettingsView /> },
                    _ => html! { <p>{ "Unknown section" }</p> },
                }}
            </div>
        </div>
    }
}
