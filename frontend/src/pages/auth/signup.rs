use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::services::auth_service::{signup, SignupCredentials, AuthError};
use crate::components::simple_notification::SimpleNotification;

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    #[allow(dead_code)]
    Info,
}

#[function_component]
pub fn SignupPage() -> Html {
    let username_ref = use_node_ref();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();
    let confirm_password_ref = use_node_ref();
    
    let notification = use_state(|| None::<(String, NotificationType)>);
    let is_loading = use_state(|| false);

    let clear_notification = {
        let notification = notification.clone();
        Callback::from(move |_| {
            notification.set(None);
        })
    };

    let on_submit = {
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let confirm_password_ref = confirm_password_ref.clone();
        let notification = notification.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let username_input = username_ref.cast::<HtmlInputElement>().unwrap();
            let email_input = email_ref.cast::<HtmlInputElement>().unwrap();
            let password_input = password_ref.cast::<HtmlInputElement>().unwrap();
            let confirm_password_input = confirm_password_ref.cast::<HtmlInputElement>().unwrap();
            
            let username = username_input.value();
            let email = email_input.value();
            let password = password_input.value();
            let confirm_password = confirm_password_input.value();

            // Validation
            if username.trim().is_empty() {
                notification.set(Some(("Username is required".to_string(), NotificationType::Error)));
                return;
            }

            if email.trim().is_empty() {
                notification.set(Some(("Email is required".to_string(), NotificationType::Error)));
                return;
            }

            if password.len() < 6 {
                notification.set(Some(("Password must be at least 6 characters".to_string(), NotificationType::Error)));
                return;
            }

            if password != confirm_password {
                notification.set(Some(("Passwords do not match".to_string(), NotificationType::Error)));
                return;
            }

            let credentials = SignupCredentials {
                username: username.clone(),
                email: email.clone(),
                password: password.clone(),
            };

            let notification = notification.clone();
            let is_loading = is_loading.clone();
            
            is_loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match signup(&credentials).await {
                    Ok(_) => {
                        notification.set(Some((
                            "Account created successfully! Please check your email to verify your account.".to_string(),
                            NotificationType::Success
                        )));
                        
                        // Clear form
                        username_input.set_value("");
                        email_input.set_value("");
                        password_input.set_value("");
                        confirm_password_input.set_value("");
                    }
                    Err(AuthError::ServerError(msg)) => {
                        notification.set(Some((msg, NotificationType::Error)));
                    }
                    Err(e) => {
                        notification.set(Some((format!("Signup failed: {}", e), NotificationType::Error)));
                    }
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <div class="auth-page">
            <div class="auth-container">
                <div class="auth-card">
                <div class="auth-header">
                    <h1>{"Create Account"}</h1>
                    <p>{"Join our community and start commenting on posts"}</p>
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

                <form onsubmit={on_submit} class="auth-form">
                    <div class="form-group">
                        <label for="username">{"Username"}</label>
                        <input
                            ref={username_ref}
                            type="text"
                            id="username"
                            name="username"
                            placeholder="Enter your username"
                            required=true
                            disabled={*is_loading}
                        />
                    </div>

                    <div class="form-group">
                        <label for="email">{"Email"}</label>
                        <input
                            ref={email_ref}
                            type="email"
                            id="email"
                            name="email"
                            placeholder="Enter your email address"
                            required=true
                            disabled={*is_loading}
                        />
                    </div>

                    <div class="form-group">
                        <label for="password">{"Password"}</label>
                        <input
                            ref={password_ref}
                            type="password"
                            id="password"
                            name="password"
                            placeholder="Enter your password"
                            required=true
                            disabled={*is_loading}
                        />
                        <small class="help-text">{"Password must be at least 6 characters long"}</small>
                    </div>

                    <div class="form-group">
                        <label for="confirm-password">{"Confirm Password"}</label>
                        <input
                            ref={confirm_password_ref}
                            type="password"
                            id="confirm-password"
                            name="confirm-password"
                            placeholder="Confirm your password"
                            required=true
                            disabled={*is_loading}
                        />
                    </div>

                    <button 
                        type="submit" 
                        class="btn btn-primary btn-full-width"
                        disabled={*is_loading}
                    >
                        {if *is_loading { "Creating Account..." } else { "Create Account" }}
                    </button>
                </form>

                <div class="auth-footer">
                    <p>
                        {"Already have an account? "}
                        <a href="/login" class="auth-link">{"Sign in here"}</a>
                    </p>
                </div>
            </div>
            </div>
        </div>
    }
}
