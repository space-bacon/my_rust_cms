// src/frontend/components/tabbed_view.rs
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlDivElement};
use crate::services::api_service::{update_post, Post};
use yew::functional::{use_state, use_effect_with_deps};

#[function_component(TabbedView)]
pub fn tabbed_view() -> Html {
    let editor_ref = use_node_ref();
    let selected_post = use_state(|| None::<Post>);
    let content = use_state(|| String::new());

    // Initialize Quill editor
    {
        let editor_ref = editor_ref.clone();
        use_effect_with_deps(
            move |_| {
                let window = window().unwrap();
                let document = window.document().unwrap();
                let editor = editor_ref.cast::<HtmlDivElement>().unwrap();

                let quill_options = js_sys::Object::new();
                let toolbar_options = js_sys::Array::new();
                
                // Full toolbar options (bold, italic, etc.)
                let formats = js_sys::Array::new();
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
                let _quill = quill
                    .dyn_into::<js_sys::Function>()
                    .unwrap()
                    .new_with_args_and_this(
                        &editor.into(),
                        &quill_options
                    );

                || ()
            },
            (),
        );
    }

    // Save function using post updates
    let save_post = {
        let selected_post = selected_post.clone();
        let content = content.clone();
        Callback::from(move |_| {
            if let Some(mut post) = (*selected_post).clone() {
                post.content = (*content).clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(err) = update_post(post.id, &post).await {
                        log::error!("Failed to update post: {:?}", err);
                    }
                });
            }
        })
    };

    html! {
        <div class="tabbed-view">
            <div class="toolbar">
                // Define toolbar buttons (bold, italic, etc.)
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
