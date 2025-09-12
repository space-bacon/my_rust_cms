use yew::prelude::*;
use crate::services::api_service::{get_comments_with_relations, delete_comment, CommentWithRelations};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum CommentFilter {
    All,
}

#[function_component(CommentModeration)]
pub fn comment_moderation() -> Html {
    let comments = use_state(Vec::<CommentWithRelations>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let selected_comments = use_state(|| std::collections::HashSet::<i32>::new());

    // Load comments
    {
        let comments = comments.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_comments_with_relations().await {
                    Ok(fetched_comments) => {
                        comments.set(fetched_comments);
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

    let on_select_comment = {
        let selected_comments = selected_comments.clone();
        Callback::from(move |(comment_id, checked): (i32, bool)| {
            let mut current = (*selected_comments).clone();
            if checked {
                current.insert(comment_id);
            } else {
                current.remove(&comment_id);
            }
            selected_comments.set(current);
        })
    };

    let on_select_all = {
        let comments = comments.clone();
        let selected_comments = selected_comments.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut current = std::collections::HashSet::<i32>::new();
            if target.checked() {
                for comment in (*comments).iter() {
                    if let Some(id) = comment.id {
                        current.insert(id);
                    }
                }
            }
            selected_comments.set(current);
        })
    };

    let on_delete_comment = {
        let comments = comments.clone();
        let error = error.clone();
        Callback::from(move |comment_id: i32| {
            let comments = comments.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_comment(comment_id).await {
                    Ok(_) => {
                        // Remove from local state
                        let new_comments: Vec<CommentWithRelations> = (*comments)
                            .iter()
                            .filter(|c| c.id != Some(comment_id))
                            .cloned()
                            .collect();
                        comments.set(new_comments);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to delete comment: {}", e)));
                    }
                }
            });
        })
    };

    let on_bulk_delete = {
        let selected_comments = selected_comments.clone();
        let on_delete_comment = on_delete_comment.clone();
        Callback::from(move |_| {
            let selected = (*selected_comments).clone();
            for comment_id in selected {
                on_delete_comment.emit(comment_id);
            }
        })
    };

    if *loading {
        html! {
            <div class="comment-moderation">
                <div class="page-header">
                    <h1>{"Comment Moderation"}</h1>
                </div>
                <div class="loading">{"Loading comments..."}</div>
            </div>
        }
    } else {
        html! {
            <div class="comment-moderation">
                <div class="page-header">
                    <h1>{"Comment Moderation"}</h1>
                </div>

                if let Some(ref error_msg) = *error {
                    <div class="error-message">{"Error: "}{error_msg}</div>
                }

                if !(*selected_comments).is_empty() {
                    <div class="bulk-actions">
                        <span>{"Selected: "}{(*selected_comments).len()}{" comments"}</span>
                        <button class="btn btn-danger" onclick={on_bulk_delete}>{"Delete Selected"}</button>
                    </div>
                }

                <div class="comments-table">
                    <table>
                        <thead>
                            <tr>
                                <th>
                                    <input 
                                        type="checkbox" 
                                        onchange={on_select_all}
                                        checked={(*selected_comments).len() == (*comments).len() && !(*comments).is_empty()}
                                    />
                                </th>
                                <th>{"Author"}</th>
                                <th>{"Comment"}</th>
                                <th>{"Post ID"}</th>
                                <th>{"Created"}</th>
                                <th>{"Actions"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {(*comments).iter().map(|comment| {
                                let comment_id = comment.id.unwrap_or(0);
                                let is_selected = (*selected_comments).contains(&comment_id);
                                
                                let on_select = {
                                    let on_select_comment = on_select_comment.clone();
                                    let comment_id = comment_id;
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                        on_select_comment.emit((comment_id, target.checked()));
                                    })
                                };

                                let on_delete = {
                                    let on_delete_comment = on_delete_comment.clone();
                                    let comment_id = comment_id;
                                    Callback::from(move |_| on_delete_comment.emit(comment_id))
                                };

                                html! {
                                    <tr key={comment_id}>
                                        <td>
                                            <input 
                                                type="checkbox" 
                                                checked={is_selected}
                                                onchange={on_select}
                                            />
                                        </td>
                                        <td>{comment.author_username.as_ref().unwrap_or(&"Anonymous".to_string())}</td>
                                        <td class="comment-content">{&comment.content}</td>
                                        <td>{comment.post_id.map(|id| id.to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                        <td>{comment.created_at.as_ref().unwrap_or(&"N/A".to_string())}</td>
                                        <td class="actions">
                                            <button class="btn btn-small btn-danger" onclick={on_delete}>{"Delete"}</button>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </div>

                if (*comments).is_empty() {
                    <div class="empty-state">
                        <p>{"No comments found."}</p>
                    </div>
                }
            </div>
        }
    }
}