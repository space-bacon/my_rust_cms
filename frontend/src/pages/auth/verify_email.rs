use yew::prelude::*;
use web_sys::window;
use crate::services::auth_service::{verify_email, AuthError};
// Notification not used in this component
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum NotificationType {
    Success,
    Error,
    Info,
}

#[derive(Clone, PartialEq)]
pub enum VerificationState {
    Loading,
    Success,
    Error(String),
}

#[function_component]
pub fn VerifyEmailPage() -> Html {
    let verification_state = use_state(|| VerificationState::Loading);

    // Parse query parameters to get the token from the URL
    let token = use_state(|| {
        if let Some(window) = window() {
            let location = window.location();
            if let Ok(search) = location.search() {
                let query_params: HashMap<String, String> = search
                    .trim_start_matches('?')
                    .split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.split('=');
                        match (parts.next(), parts.next()) {
                            (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                            _ => None,
                        }
                    })
                    .collect();
                
                return query_params.get("token").cloned();
            }
        }
        None
    });

    // Verify email on component mount
    {
        let verification_state = verification_state.clone();
        let token = token.clone();
        
        use_effect(move || {
            if let Some(token) = (*token).clone() {
                let verification_state = verification_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match verify_email(&token).await {
                        Ok(_) => {
                            verification_state.set(VerificationState::Success);
                        }
                        Err(AuthError::ServerError(msg)) => {
                            verification_state.set(VerificationState::Error(msg));
                        }
                        Err(e) => {
                            verification_state.set(VerificationState::Error(format!("Verification failed: {}", e)));
                        }
                    }
                });
            } else {
                verification_state.set(VerificationState::Error("No verification token provided".to_string()));
            }
        });
    }

    html! {
        <div class="auth-page">
            <div class="auth-container">
                <div class="auth-card">
                <div class="auth-header">
                    <h1>{"Email Verification"}</h1>
                </div>

                <div class="verification-content">
                    {
                        match &*verification_state {
                            VerificationState::Loading => html! {
                                <div class="loading-state">
                                    <div class="spinner"></div>
                                    <p>{"Verifying your email address..."}</p>
                                </div>
                            },
                            VerificationState::Success => html! {
                                <div class="success-state">
                                    <div class="success-icon">{"✓"}</div>
                                    <h2>{"Email Verified Successfully!"}</h2>
                                    <p>{"Your email address has been verified. Your account is now active and you can log in."}</p>
                                    <div class="action-buttons">
                                        <a href="/login" class="btn btn-primary">{"Go to Login"}</a>
                                    </div>
                                </div>
                            },
                            VerificationState::Error(message) => html! {
                                <div class="error-state">
                                    <div class="error-icon">{"✗"}</div>
                                    <h2>{"Verification Failed"}</h2>
                                    <p>{message.clone()}</p>
                                    <div class="action-buttons">
                                        <a href="/signup" class="btn btn-secondary">{"Sign Up Again"}</a>
                                        <a href="/login" class="btn btn-primary">{"Try Login"}</a>
                                    </div>
                                </div>
                            }
                        }
                    }
                </div>
            </div>
            </div>
        </div>
    }
}
