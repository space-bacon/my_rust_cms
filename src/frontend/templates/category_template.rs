use yew::prelude::*;

#[function_component(CategoryTemplate)]
pub fn category_template() -> Html {
    html! {
        <div class="category-template">
            <h1>{ "Category Name" }</h1>
            <ul>
                <li>{ "Post 1" }</li>
                <li>{ "Post 2" }</li>
                <li>{ "Post 3" }</li>
            </ul>
        </div>
    }
}
