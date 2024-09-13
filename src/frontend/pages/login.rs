use yew::prelude::*;
use yew::events::SubmitEvent;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());

    let on_submit = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            log::info!("Username: {}, Password: {}", *username, *password);
        })
    };

    html! {
        <div class="login">
            <h2>{ "Login" }</h2>
            <form onsubmit={on_submit}>
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
                <button type="submit">{ "Login" }</button>
            </form>
        </div>
    }
}
