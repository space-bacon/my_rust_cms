use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use crate::components::comment_item::CommentItem;
use crate::services::api_service::{CommentWithGravatar, PublicCommentRequest, get_post_comments, get_page_comments, create_public_comment};
use crate::services::auth_service::{get_current_user, User};
use crate::components::simple_notification::SimpleNotification;

#[derive(Properties, PartialEq)]
pub struct CommentsSectionProps {
    pub post_id: Option<i32>,
    pub page_id: Option<i32>,
    pub show_auth_prompt: bool,
}

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    #[allow(dead_code)]
    Info,
}

#[function_component(CommentsSection)]
pub fn comments_section(props: &CommentsSectionProps) -> Html {
    let comments = use_state(Vec::<CommentWithGravatar>::new);
    let loading = use_state(|| true);
    let current_user = use_state(|| None::<User>);
    let comment_text = use_state(String::new);
    let submitting = use_state(|| false);
    let notification = use_state(|| None::<(String, NotificationType)>);
    let show_login_form = use_state(|| false);
    let show_signup_form = use_state(|| false);
    
    let comment_ref = use_node_ref();

    // Load comments and current user on mount
    {
        let comments = comments.clone();
        let loading = loading.clone();
        let current_user = current_user.clone();
        let post_id = props.post_id;
        let page_id = props.page_id;

        use_effect_with_deps(move |_| {
            let comments = comments.clone();
            let loading = loading.clone();
            let current_user = current_user.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Load comments for either post or page
                let result = if let Some(post_id) = post_id {
                    get_post_comments(post_id).await
                } else if let Some(page_id) = page_id {
                    get_page_comments(page_id).await
                } else {
                    Ok(vec![]) // No ID provided, return empty
                };

                match result {
                    Ok(fetched_comments) => {
                        comments.set(fetched_comments);
                    }
                    Err(_) => {
                        // Handle error silently or show notification
                    }
                }

                // Check if user is logged in
                match get_current_user().await {
                    Ok(user) => {
                        current_user.set(Some(user));
                    }
                    Err(_) => {
                        current_user.set(None);
                    }
                }

                loading.set(false);
            });
            || ()
        }, ());
    }

    let clear_notification = {
        let notification = notification.clone();
        Callback::from(move |_| {
            notification.set(None);
        })
    };

    let on_comment_change = {
        let comment_text = comment_text.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            comment_text.set(input.value());
        })
    };

    // Extract props fields to avoid lifetime issues
    let current_post_id = props.post_id;
    let current_page_id = props.page_id;

    let submit_comment = {
        let comment_text = comment_text.clone();
        let current_user = current_user.clone();
        let comments = comments.clone();
        let submitting = submitting.clone();
        let notification = notification.clone();
        let comment_ref = comment_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let text = (*comment_text).clone();
            if text.trim().is_empty() {
                notification.set(Some(("Please enter a comment".to_string(), NotificationType::Error)));
                return;
            }

            if let Some(user) = (*current_user).clone() {
                let comment_request = PublicCommentRequest {
                    content: text.trim().to_string(),
                    post_id: current_post_id,
                    page_id: current_page_id,
                    user_id: user.id,
                };

                let comment_text = comment_text.clone();
                let comments = comments.clone();
                let submitting = submitting.clone();
                let notification = notification.clone();
                let comment_ref = comment_ref.clone();

                submitting.set(true);

                wasm_bindgen_futures::spawn_local(async move {
                    match create_public_comment(&comment_request).await {
                        Ok(new_comment) => {
                            let mut current_comments = (*comments).clone();
                            current_comments.push(new_comment);
                            comments.set(current_comments);
                            comment_text.set(String::new());
                            
                            // Clear the textarea
                            if let Some(textarea) = comment_ref.cast::<HtmlTextAreaElement>() {
                                textarea.set_value("");
                            }

                            notification.set(Some(("Comment posted successfully!".to_string(), NotificationType::Success)));
                        }
                        Err(e) => {
                            notification.set(Some((format!("Failed to post comment: {}", e), NotificationType::Error)));
                        }
                    }
                    submitting.set(false);
                });
            }
        })
    };

    let toggle_login = {
        let show_login_form = show_login_form.clone();
        let show_signup_form = show_signup_form.clone();
        Callback::from(move |_| {
            show_signup_form.set(false);
            show_login_form.set(!*show_login_form);
        })
    };

    let toggle_signup = {
        let show_login_form = show_login_form.clone();
        let show_signup_form = show_signup_form.clone();
        Callback::from(move |_| {
            show_login_form.set(false);
            show_signup_form.set(!*show_signup_form);
        })
    };

    if *loading {
        return html! {
            <div class="comments-section loading">
                <div class="loading-spinner">{"Loading comments..."}</div>
            </div>
        };
    }

    html! {
        <div class="comments-section">
            <div class="comments-header">
                <h3>{"Comments"} <span class="comment-count">{format!("({})", comments.len())}</span></h3>
            </div>

            {
                if let Some((message, notification_type)) = (*notification).clone() {
                    let class = match notification_type {
                        NotificationType::Success => "notification-success",
                        NotificationType::Error => "notification-error",
                        NotificationType::Info => "notification-info",
                    };
                    html! {
                        <SimpleNotification 
                            message={message} 
                            notification_type={class} 
                            on_close={clear_notification.clone()}
                        />
                    }
                } else {
                    html! {}
                }
            }

            // Comment form
            <div class="comment-form-container">
                {
                    if let Some(user) = (*current_user).clone() {
                        html! {
                            <form class="comment-form" onsubmit={submit_comment}>
                                <div class="comment-form-header">
                                    <img 
                                        src={format!("https://www.gravatar.com/avatar/{}?s=40&d=identicon&r=pg", 
                                            md5::compute(user.email.to_lowercase().as_bytes()).iter()
                                                .map(|b| format!("{:02x}", b)).collect::<String>()
                                        )}
                                        alt="Your avatar"
                                        class="comment-form-avatar"
                                    />
                                    <span class="comment-form-user">{"Commenting as "}<strong>{user.username}</strong></span>
                                </div>
                                <div class="comment-form-input">
                                    <textarea
                                        ref={comment_ref}
                                        value={(*comment_text).clone()}
                                        oninput={on_comment_change}
                                        placeholder="Share your thoughts..."
                                        rows="4"
                                        disabled={*submitting}
                                        class="comment-textarea"
                                    />
                                </div>
                                <div class="comment-form-actions">
                                    <button 
                                        type="submit" 
                                        class="btn btn-primary"
                                        disabled={*submitting || comment_text.trim().is_empty()}
                                    >
                                        {if *submitting { "Posting..." } else { "Post Comment" }}
                                    </button>
                                </div>
                            </form>
                        }
                    } else {
                        html! {
                            <div class="comment-auth-prompt">
                                <div class="auth-prompt-content">
                                    <p>{"Join the conversation! Sign in or create an account to leave a comment."}</p>
                                    <div class="auth-prompt-actions">
                                        <button 
                                            class="btn btn-outline-primary" 
                                            onclick={toggle_login}
                                        >
                                            {"Sign In"}
                                        </button>
                                        <button 
                                            class="btn btn-primary" 
                                            onclick={toggle_signup}
                                        >
                                            {"Create Account"}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                }
            </div>

            // Comments list
            <div class="comments-list">
                {
                    if comments.is_empty() {
                        html! {
                            <div class="no-comments">
                                <p>{"No comments yet. Be the first to share your thoughts!"}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <>
                                {for comments.iter().map(|comment| {
                                    html! {
                                        <CommentItem comment={comment.clone()} />
                                    }
                                })}
                            </>
                        }
                    }
                }
            </div>
        </div>
    }
}
