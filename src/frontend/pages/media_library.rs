use yew::prelude::*;

#[function_component(MediaLibrary)]
pub fn media_library() -> Html {
    html! {
        <div class="media-library">
            <h2>{ "Media Library" }</h2>
            { "Here you can manage your media files." }
        </div>
    }
}
