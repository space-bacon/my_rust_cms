use yew::prelude::*;
use crate::frontend::services::api_service::{get_posts, Post};
use crate::frontend::components::tabbed_view::TabbedView;

#[function_component(PostExplorer)]
pub fn post_explorer() -> Html {
    let posts = use_state(|| Vec::new());
    let selected_post = use_state(|| None::<Post>);

    {
        let posts = posts.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_posts().await {
                    Ok(fetched_posts) => posts.set(fetched_posts),
                    Err(err) => log::error!("Error getting posts: {:?}", err),
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
                        let post_display = post.clone(); // Clone for display
                        let post_clone = post.clone(); // Clone for closure
                        let on_post_click = on_post_click.clone();
                        html! {
                            <li onclick={move |_| on_post_click.emit(post_clone.clone())}>
                                { format!("{} ({})", post_display.title, post_display.category) }
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
