// src/frontend/components/post_explorer.rs
use crate::services::api_service::{get_posts, Post};
use yew::prelude::*;
use yew::functional::{use_state, use_effect_with_deps};

#[function_component(PostExplorer)]
pub fn post_explorer() -> Html {
    let posts = use_state(|| vec![]);

    // Fetch posts on component mount
    {
        let posts_clone = posts.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get_posts().await {
                        Ok(fetched_posts) => posts_clone.set(fetched_posts),
                        Err(err) => log::error!("Failed to fetch posts: {:?}", err),
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="post-explorer">
            <h2>{ "Posts" }</h2>
            <ul>
                { for posts.iter().map(|post| html! {
                    <li key={post.id}>{ &post.title }</li>
                })}
            </ul>
        </div>
    }
}
