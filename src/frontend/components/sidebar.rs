use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub on_select: Callback<String>, // Callback for parent to know which section is clicked
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let on_post_click = {
        let on_select = props.on_select.clone();
        Callback::from(move |_| on_select.emit("post".to_string()))
    };

    let on_media_click = {
        let on_select = props.on_select.clone();
        Callback::from(move |_| on_select.emit("media".to_string()))
    };

    let on_settings_click = {
        let on_select = props.on_select.clone();
        Callback::from(move |_| on_select.emit("settings".to_string()))
    };

    html! {
        <div class="sidebar">
            <button onclick={on_post_click}>{"📄"}</button>
            <button onclick={on_media_click}>{"📷"}</button>
            <button onclick={on_settings_click}>{"⚙️"}</button>
        </div>
    }
}
