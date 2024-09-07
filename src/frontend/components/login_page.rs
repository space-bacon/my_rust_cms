use yew::prelude::*;
use crate::frontend::services::api_service::{login, AuthData};
use yew::functional::use_state;
use yew::events::InputEvent;
use web_sys::HtmlInputElement;
use log::info;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let login_error = use_state(|| None::<String>);

    let on_login_click = {
        let username = username.clone();
        let password = password.clone();
        let login_error = login_error.clone();

        Callback::from(move |_| {
            let auth_data = AuthData {
                username: (*username).clone(),
                password: (*password).clone(),
            };

            let login_error = login_error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match login(auth_data).await {
                    Ok(_) => {
                        info!("Login successful!");
                    },
                    Err(err) => {
                        login_error.set(Some(format!("Login failed: {:?}", err)));
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Login" }</h1>
            <input 
                type="text" 
                placeholder="Username" 
                value={(*username).clone()} 
                oninput={Callback::from(move |e: InputEvent| username.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
            />
            <input 
                type="password" 
                placeholder="Password" 
                value={(*password).clone()} 
                oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
            />
            <button onclick={on_login_click}>{ "Login" }</button>
            if let Some(error) = (*login_error).clone() {
                <p style="color: red;">{ error }</p>
            }
        </div>
    }
}
