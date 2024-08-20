use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CommentItemProps {
    pub author: String,
    pub content: String,
}

#[function_component(CommentItem)]
pub fn comment_item(props: &CommentItemProps) -> Html {
    html! {
        <div class="comment-item">
            <strong>{ &props.author }</strong>
            <p>{ &props.content }</p>
        </div>
    }
}
