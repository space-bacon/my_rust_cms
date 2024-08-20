use yew::prelude::*;

#[function_component(Toolbox)]
pub fn toolbox() -> Html {
    html! {
        <div class="toolbox">
            <h3>{ "Modules" }</h3>
            <ul>
                <li>{ "Text" }</li>
                <li>{ "Image" }</li>
                <li>{ "Button" }</li>
            </ul>
        </div>
    }
}
