use yew::prelude::*;
use crate::services::api_service::{get_posts, delete_post, Post};
use crate::components::admin::sidebar::AdminTab;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum PostListView {
    List,
    Create,
    Edit(Post),
}

#[derive(Properties, PartialEq)]
pub struct PostListProps {
    #[prop_or_default]
    pub on_navigate: Option<Callback<AdminTab>>,
}

#[function_component(PostList)]
pub fn post_list(props: &PostListProps) -> Html {
    let posts = use_state(Vec::<Post>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let current_view = use_state(|| PostListView::List);

    {
        let posts = posts.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_posts().await {
                    Ok(fetched_posts) => {
                        posts.set(fetched_posts);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    let on_delete_post = {
        let posts = posts.clone();
        let error = error.clone();
        Callback::from(move |post_id: i32| {
            let posts = posts.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_post(post_id).await {
                    Ok(_) => {
                        // Remove the deleted post from the list
                        let mut current_posts = (*posts).clone();
                        current_posts.retain(|post| post.id != Some(post_id));
                        posts.set(current_posts);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to delete post: {}", e)));
                    }
                }
            });
        })
    };

    let on_create_post = {
        let current_view = current_view.clone();
        Callback::from(move |_| {
            current_view.set(PostListView::Create);
        })
    };

    let on_edit_post = {
        let current_view = current_view.clone();
        Callback::from(move |post: Post| current_view.set(PostListView::Edit(post)))
    };

    let on_save_post = {
        let posts = posts.clone();
        let current_view = current_view.clone();
        Callback::from(move |saved_post: Post| {
            let mut current_posts: Vec<Post> = (*posts).clone();
            if let Some(existing_index) = current_posts.iter().position(|p| p.id == saved_post.id) {
                current_posts[existing_index] = saved_post;
            } else {
                current_posts.push(saved_post);
            }
            posts.set(current_posts);
            current_view.set(PostListView::List);
        })
    };

    let on_cancel_edit = {
        let current_view = current_view.clone();
        Callback::from(move |_| current_view.set(PostListView::List))
    };

    match *current_view {
        PostListView::List => {
            let on_create_post_clone = on_create_post.clone();
            html! {
                <div class="post-list">
                    <div class="page-header">
                        <div>
                            <h1>{"Posts"}</h1>
                            <p>{"Manage and organize your content"}</p>
                        </div>
                        <div class="header-actions">
                            <button class="btn btn-primary" onclick={on_create_post}>{"Add New Post"}</button>
                        </div>
                    </div>

                    if *loading {
                        <div class="loading">{"Loading posts..."}</div>
                    } else if let Some(ref error_msg) = *error {
                        <div class="error">{"Error loading posts: "}{error_msg}</div>
                    } else if posts.is_empty() {
                        <div class="empty-state">
                            <h3>{"No posts yet"}</h3>
                            <p>{"Create your first post to get started!"}</p>
                            <button class="btn" onclick={on_create_post_clone}>{"Create Post"}</button>
                        </div>
                } else {
                    <div class="admin-table-container">
                        <table>
                            <thead>
                                <tr>
                                    <th>{"Title"}</th>
                                    <th>{"Author"}</th>
                                    <th>{"Status"}</th>
                                    <th>{"Date"}</th>
                                    <th>{"Actions"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {posts.iter().map(|post| {
                                    let on_delete = {
                                        let on_delete_post = on_delete_post.clone();
                                        let post_id = post.id.unwrap_or(0);
                                        Callback::from(move |_| on_delete_post.emit(post_id))
                                    };

                                    let on_edit = {
                                        let on_edit_post = on_edit_post.clone();
                                        let post = post.clone();
                                        Callback::from(move |_| on_edit_post.emit(post.clone()))
                                    };

                                    html! {
                                        <tr>
                                            <td>{&post.title}</td>
                                            <td>{&post.author}</td>
                                            <td>
                                                <span class={classes!("status-badge", if post.status == "published" { "published" } else { "draft" })}>
                                                    {&post.status}
                                                </span>
                                            </td>
                                            <td>{post.created_at.as_deref().unwrap_or("Unknown")}</td>
                                            <td class="actions">
                                                <button class="btn btn-secondary" onclick={on_edit}>{"Edit"}</button>
                                                <button class="btn btn-danger" onclick={on_delete}>{"Delete"}</button>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()}
                            </tbody>
                        </table>
                    </div>
                }
            </div>
            }
        },
        PostListView::Create => html! {
            <crate::pages::admin::PostEditor
                post={None}
                on_save={on_save_post}
                on_cancel={on_cancel_edit}
            />
        },
        PostListView::Edit(ref post) => html! {
            <crate::pages::admin::PostEditor
                post={Some(post.clone())}
                on_save={on_save_post}
                on_cancel={on_cancel_edit}
            />
        },
    }
} 