use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <aside>
            <nav>
                <ul>
                    <li>{ "Dashboard" }</li>
                    <li>{ "Posts" }</li>
                    <li>{ "Media" }</li>
                    <li>{ "Settings" }</li>
                </ul>
            </nav>
        </aside>
    }
}
