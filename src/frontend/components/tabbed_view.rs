use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::{JsValue, closure::Closure};
use web_sys::{window, HtmlElement};
use crate::frontend::services::api_service::{update_post, Post};
use yew::functional::{use_state, use_effect_with_deps};
use js_sys::{Object, Array};

#[function_component(TabbedView)]
pub fn tabbed_view(props: &Props) -> Html {
    let Props { post } = props.clone();
    let editor_ref = use_node_ref();
    let content = use_state(|| post.content.clone());

    // Initialize Quill editor
    {
        let editor_ref = editor_ref.clone();
        let content = content.clone();
        use_effect_with_deps(
            move |_| {
                let window = window().unwrap();
                let editor = editor_ref.cast::<HtmlElement>().unwrap();
                
                let quill_options = Object::new();
                let toolbar_options = Array::new();

                // Full toolbar options
                let formats = Array::new();
                formats.push(&JsValue::from_str("bold"));
                formats.push(&JsValue::from_str("italic"));
                formats.push(&JsValue::from_str("underline"));
                formats.push(&JsValue::from_str("strike"));
                formats.push(&JsValue::from_str("link"));
                formats.push(&JsValue::from_str("header"));

                js_sys::Reflect::set(
                    &quill_options,
                    &JsValue::from_str("theme"),
                    &JsValue::from_str("snow")
                ).unwrap();
                js_sys::Reflect::set(
                    &quill_options,
                    &JsValue::from_str("modules"),
                    &formats.into()
                ).unwrap();

                let quill = js_sys::Reflect::get(&window, &JsValue::from_str("Quill")).unwrap();
                let quill_instance = quill
                    .dyn_into::<js_sys::Function>()
                    .unwrap()
                    .call2(&JsValue::NULL, &editor.clone().into(), &quill_options)
                    .unwrap();

                let editor_cloned = editor.clone();  // Cloning editor for closure
                let content_cloned = content.clone();

                // Set up listener for Quill content updates
                let callback = Closure::wrap(Box::new(move || {
                    let inner_html = editor_cloned.inner_html();
                    content_cloned.set(inner_html);
                }) as Box<dyn Fn()>);
                
                quill_instance
                    .unchecked_ref::<js_sys::Function>()
                    .call1(&JsValue::NULL, &callback.into_js_value())
                    .unwrap();

                || ()
            },
            (),
        );
    }

    // Save function using post updates
    let save_post = {
        let post = post.clone();
        let content = content.clone();
        Callback::from(move |_| {
            let mut updated_post = post.clone();
            updated_post.content = (*content).clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(err) = update_post(updated_post.id, &updated_post).await {
                    log::error!("Failed to update post: {:?}", err);
                }
            });
        })
    };

    html! {
        <div class="tabbed-view">
            <div class="toolbar">
                <button>{ "B" }</button>
                <button>{ "I" }</button>
                <button>{ "U" }</button>
                <button>{ "Link" }</button>
            </div>
            <div ref={editor_ref} class="quill-editor" style="min-height: 300px;"></div>
            <button onclick={save_post}>{ "Save" }</button>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub post: Post,
}
