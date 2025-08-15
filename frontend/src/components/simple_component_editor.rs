use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element};

#[derive(Properties, PartialEq, Clone)]
pub struct SimpleComponentEditorProps {
    pub enabled: bool,
}

#[function_component(SimpleComponentEditor)]
pub fn simple_component_editor(props: &SimpleComponentEditorProps) -> Html {
    let selected_element = use_state(|| None::<String>);
    let show_panel = use_state(|| false);
    let element_content = use_state(|| String::new());

    // Setup component selection when enabled
    {
        let selected_element = selected_element.clone();
        let show_panel = show_panel.clone();
        let element_content = element_content.clone();
        
        use_effect_with_deps(move |enabled| {
            if *enabled {
                if let Some(doc) = window().and_then(|w| w.document()) {
                    // Add selection outlines to page components
                    let components_with_class = doc.query_selector_all(".component").unwrap();
                    for i in 0..components_with_class.length() {
                        if let Some(node) = components_with_class.item(i) {
                            if let Ok(el) = node.dyn_into::<Element>() {
                                let _ = el.set_attribute("data-live-editable", "true");
                                if let Some(existing) = el.get_attribute("style") {
                                    let _ = el.set_attribute("style", &format!("{}; outline: 2px dashed #0066cc; cursor: pointer;", existing));
                                } else {
                                    let _ = el.set_attribute("style", "outline: 2px dashed #0066cc; cursor: pointer;");
                                }

                                // Add click handler
                                let element_id = format!("component-{}", i);
                                let _ = el.set_attribute("data-component-id", &element_id);
                                
                                let selected_element_clone = selected_element.clone();
                                let show_panel_clone = show_panel.clone();
                                let element_content_clone = element_content.clone();
                                
                                // Note: In a real implementation, you'd need to properly handle event listeners
                                // For now, this demonstrates the concept
                                if let Some(text_content) = el.text_content() {
                                    element_content_clone.set(text_content);
                                }
                            }
                        }
                    }
                }
            }

            // Cleanup function
            || {
                if let Some(doc) = window().and_then(|w| w.document()) {
                    let components_with_class = doc.query_selector_all("[data-live-editable]").unwrap();
                    for i in 0..components_with_class.length() {
                        if let Some(node) = components_with_class.item(i) {
                            if let Ok(el) = node.dyn_into::<Element>() {
                                let _ = el.remove_attribute("data-live-editable");
                                let _ = el.remove_attribute("data-component-id");
                                if let Some(existing) = el.get_attribute("style") {
                                    let cleaned = existing
                                        .replace("outline: 2px dashed #0066cc;", "")
                                        .replace("cursor: pointer;", "");
                                    let _ = el.set_attribute("style", cleaned.trim());
                                }
                            }
                        }
                    }
                }
            }
        }, props.enabled);
    }

    let on_close = {
        let show_panel = show_panel.clone();
        let selected_element = selected_element.clone();
        Callback::from(move |_: MouseEvent| {
            show_panel.set(false);
            selected_element.set(None);
        })
    };

    let on_edit_click = {
        let show_panel = show_panel.clone();
        Callback::from(move |_: MouseEvent| {
            show_panel.set(true);
        })
    };

    if !props.enabled {
        return html! {};
    }

    html! {
        <div class="simple-component-editor">
            // Live edit indicator
            <div style="position: fixed; top: 10px; right: 10px; z-index: 10000;">
                <div style="background: rgba(0,0,0,0.8); color: white; padding: 8px 12px; border-radius: 6px; font-size: 12px;">
                    {"🎨 Live Edit Mode Active - Click components to edit"}
                </div>
                
                if *show_panel {
                    <div style="position: fixed; top: 60px; right: 10px; background: white; border: 1px solid #ddd; border-radius: 8px; padding: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); min-width: 300px; z-index: 10001;">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                            <h3 style="margin: 0; font-size: 14px;">{"Component Editor"}</h3>
                            <button onclick={on_close.clone()} style="background: none; border: none; font-size: 16px; cursor: pointer;">{"×"}</button>
                        </div>
                        
                        <div style="margin-bottom: 12px;">
                            <label style="display: block; margin-bottom: 4px; font-size: 12px; font-weight: bold;">{"Content:"}</label>
                            <textarea 
                                value={(*element_content).clone()}
                                rows="4"
                                style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 13px; resize: vertical;"
                                placeholder="Edit component content..."
                            />
                        </div>
                        
                        <div style="display: flex; gap: 8px; justify-content: flex-end;">
                            <button 
                                style="padding: 6px 12px; background: #f5f5f5; border: 1px solid #ddd; border-radius: 4px; cursor: pointer; font-size: 12px;"
                                onclick={on_close}
                            >
                                {"Cancel"}
                            </button>
                            <button 
                                style="padding: 6px 12px; background: #0066cc; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;"
                            >
                                {"Save Changes"}
                            </button>
                        </div>
                    </div>
                } else {
                    <button 
                        onclick={on_edit_click}
                        style="margin-top: 8px; padding: 6px 12px; background: #0066cc; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;"
                    >
                        {"Edit Component"}
                    </button>
                }
            </div>
        </div>
    }
}
