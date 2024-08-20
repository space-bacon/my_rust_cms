use yew::prelude::*;

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    html! {
        <div class="user-management">
            <h2>{ "User Management" }</h2>
            { "Manage your users here." }
        </div>
    }
}
