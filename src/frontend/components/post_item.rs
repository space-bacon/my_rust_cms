use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostItemProps {
    pub title: String,
    pub summary: String,
}

#[function_component(PostItem)]
pub fn post_item(props: &PostItemProps) -> Html {
    html! {
        <div class="post-item">
            <h2>{ &props.title }</h2>
            <p>{ &props.summary }</p>
        </div>
    }
}
