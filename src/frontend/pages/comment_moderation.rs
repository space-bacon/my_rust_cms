use yew::prelude::*;

#[function_component(CommentModeration)]
pub fn comment_moderation() -> Html {
    html! {
        <div class="comment-moderation">
            <h2>{ "Comment Moderation" }</h2>
            { "Moderate comments here." }
        </div>
    }
}
