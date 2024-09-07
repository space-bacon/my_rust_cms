// src/frontend/components/post_explorer.rs
use gloo_net::http::Request;
use yew::prelude::*;
use serde::Deserialize;
use crate::services::api_service::fetch_posts;
use crate::components::tabbed_view::TabbedView;

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    id: i32,
    title: String,
    category: String,
    content: String,
}

#[function_component(PostExplorer)]
pub fn post_explorer() -> Html {
    let posts = use_state(|| Vec::new());
    let selected_post = use_state(|| None::<Post>);

    {
        let posts = posts.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_posts().await {
                    Ok(fetched_posts) => posts.set(fetched_posts),
                    Err(err) => log::error!("Error fetching posts: {:?}", err),
                }
            });
            || ()
        }, ());
    }

    let on_post_click = {
        let selected_post = selected_post.clone();
        Callback::from(move |post: Post| {
            selected_post.set(Some(post));
        })
    };

    html! {
        <div class="post-explorer">
            <h3>{ "Posts" }</h3>
            <ul>
                {
                    for posts.iter().cloned().map(|post| {
                        let on_post_click = on_post_click.clone();
                        html! {
                            <li onclick={move |_| on_post_click.emit(post.clone())}>
                                { format!("{} ({})", post.title, post.category) }
                            </li>
                        }
                    })
                }
            </ul>
            { selected_post.as_ref().map(|post| html! {
                <TabbedView post={post.clone()} />
            }) }
        </div>
    }
}
