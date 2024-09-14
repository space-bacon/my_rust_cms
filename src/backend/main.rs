use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/posts")]
    Posts,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! { <h1>{ "Home Page" }</h1> }
}

#[function_component(Posts)]
fn posts() -> Html {
    html! { <h1>{ "Posts Page" }</h1> }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Posts => html! { <Posts /> },
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::Posts}>{ "Posts" }</Link<Route>>
            </nav>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::start_app::<App>();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This app is intended to run as a WebAssembly app.");
}
