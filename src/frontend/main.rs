use yew::prelude::*;
use crate::pages::page_builder::PageBuilder;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <Sidebar />
            <div class="content">
                <PageBuilder />
            </div>
            <Footer />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
