use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="login">
            <h2>{ "Login" }</h2>
            <form>
                <input type="text" placeholder="Username" />
                <input type="password" placeholder="Password" />
                <button type="submit">{ "Login" }</button>
            </form>
        </div>
    }
}
