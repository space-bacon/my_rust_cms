use yew::prelude::*;
use crate::services::api_service::{get_media, MediaItem};
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct MediaPickerProps {
    pub on_select: Callback<MediaItem>,
    pub on_multi_select: Option<Callback<Vec<MediaItem>>>,
    pub on_close: Callback<()>,
    pub show: bool,
    pub filter_images_only: bool,
    pub allow_multi_select: bool,
}

#[function_component(MediaPicker)]
pub fn media_picker(props: &MediaPickerProps) -> Html {
    let media_items = use_state(Vec::<MediaItem>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let search_term = use_state(|| String::new());
    let selected_items = use_state(Vec::<MediaItem>::new);

    // Load media on mount
    {
        let media_items = media_items.clone();
        let loading = loading.clone();
        let error = error.clone();
        let show = props.show;

        use_effect_with_deps(move |_| {
            if show {
                wasm_bindgen_futures::spawn_local(async move {
                    match get_media().await {
                        Ok(fetched_media) => {
                            media_items.set(fetched_media);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to load media: {}", e)));
                            loading.set(false);
                        }
                    }
                });
            }
            || ()
        }, show);
    }

    // Filter media items based on search and type
    let filtered_items = {
        let filter_images_only = props.filter_images_only;
        
        use_memo(
            move |(items, search)| {
                items.iter()
                    .filter(|item| {
                        // Filter by type if needed
                        if filter_images_only && !item.type_.starts_with("image") {
                            return false;
                        }
                        // When not filtering images only, show videos primarily but also allow images
                        if !filter_images_only && !item.type_.starts_with("video") && !item.type_.starts_with("image") {
                            return false;
                        }
                        // Filter by search term
                        if !search.is_empty() {
                            item.name.to_lowercase().contains(&search.to_lowercase())
                        } else {
                            true
                        }
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            },
            ((*media_items).clone(), (*search_term).clone())
        )
    };

    let on_search_input = {
        let search_term = search_term.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            search_term.set(target.value());
        })
    };

    let backdrop_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
            if target.class_name().contains("media-picker-backdrop") {
                on_close.emit(());
            }
        })
    };

    let toggle_item_selection = {
        let selected_items = selected_items.clone();
        Callback::from(move |item: MediaItem| {
            let mut current_selected = (*selected_items).clone();
            if let Some(pos) = current_selected.iter().position(|x| x.id == item.id) {
                // Item is already selected, remove it
                current_selected.remove(pos);
            } else {
                // Item is not selected, add it
                current_selected.push(item);
            }
            selected_items.set(current_selected);
        })
    };

    let confirm_multi_selection = {
        let selected_items = selected_items.clone();
        let on_multi_select = props.on_multi_select.clone();
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            if let Some(ref callback) = on_multi_select {
                callback.emit((*selected_items).clone());
            }
            on_close.emit(());
        })
    };

    let clear_selection = {
        let selected_items = selected_items.clone();
        Callback::from(move |_| {
            selected_items.set(Vec::new());
        })
    };

    if !props.show {
        return html! {};
    }

    html! {
        <div class="media-picker-backdrop" onclick={backdrop_click} style="
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.5);
            z-index: 150000;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        ">
            <div class="media-picker-modal" style="
                background: white;
                border-radius: 12px;
                box-shadow: 0 25px 50px rgba(0, 0, 0, 0.25);
                max-width: 800px;
                width: 100%;
                max-height: 80vh;
                overflow: hidden;
                display: flex;
                flex-direction: column;
            ">
                <div class="media-picker-header" style="
                    padding: 20px 24px;
                    border-bottom: 1px solid #e1e5e9;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                ">
                    <h3 style="margin: 0; font-size: 18px; font-weight: 600; color: #333;">
                        {if props.allow_multi_select {
                            if props.filter_images_only { 
                                format!("Select Images ({})", selected_items.len())
                            } else { 
                                format!("Select Media ({})", selected_items.len())
                            }
                        } else {
                            if props.filter_images_only { "Select Image".to_string() } else { "Select Video".to_string() }
                        }}
                    </h3>
                    <button 
                        onclick={let on_close = props.on_close.clone(); Callback::from(move |_| on_close.emit(()))}
                        style="
                            background: none;
                            border: none;
                            font-size: 24px;
                            cursor: pointer;
                            color: #666;
                            padding: 0;
                            width: 32px;
                            height: 32px;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            border-radius: 6px;
                        "
                        title="Close"
                    >
                        {"×"}
                    </button>
                </div>

                <div class="media-picker-controls" style="
                    padding: 16px 24px;
                    border-bottom: 1px solid #e1e5e9;
                ">
                    <input 
                        type="text"
                        placeholder="Search media..."
                        value={(*search_term).clone()}
                        oninput={on_search_input}
                        style="
                            width: 100%;
                            padding: 10px 12px;
                            border: 1px solid #ddd;
                            border-radius: 6px;
                            font-size: 14px;
                            margin-bottom: 12px;
                        "
                    />
                    
                    {if props.allow_multi_select {
                        html! {
                            <div class="multi-select-actions" style="
                                display: flex;
                                gap: 8px;
                                align-items: center;
                                justify-content: space-between;
                            ">
                                <div style="
                                    font-size: 14px;
                                    color: #666;
                                ">
                                    {if selected_items.is_empty() {
                                        "Click items to select multiple".to_string()
                                    } else {
                                        format!("{} item{} selected", selected_items.len(), if selected_items.len() == 1 { "" } else { "s" })
                                    }}
                                </div>
                                <div style="display: flex; gap: 8px;">
                                    {if !selected_items.is_empty() {
                                        html! {
                                            <>
                                                <button
                                                    onclick={clear_selection.clone()}
                                                    style="
                                                        padding: 6px 12px;
                                                        background: #f8f9fa;
                                                        border: 1px solid #dee2e6;
                                                        border-radius: 4px;
                                                        font-size: 12px;
                                                        cursor: pointer;
                                                        color: #6c757d;
                                                    "
                                                >
                                                    {"Clear"}
                                                </button>
                                                <button
                                                    onclick={confirm_multi_selection.clone()}
                                                    style="
                                                        padding: 6px 16px;
                                                        background: #007bff;
                                                        border: 1px solid #007bff;
                                                        border-radius: 4px;
                                                        font-size: 12px;
                                                        cursor: pointer;
                                                        color: white;
                                                        font-weight: 500;
                                                    "
                                                >
                                                    {format!("Add {} Item{}", selected_items.len(), if selected_items.len() == 1 { "" } else { "s" })}
                                                </button>
                                            </>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>

                <div class="media-picker-content" style="
                    flex: 1;
                    overflow-y: auto;
                    padding: 20px 24px;
                ">
                    {if *loading {
                        html! {
                            <div class="loading-state" style="
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                padding: 40px;
                                color: #666;
                            ">
                                {"Loading media..."}
                            </div>
                        }
                    } else if let Some(ref error_msg) = *error {
                        html! {
                            <div class="error-state" style="
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                padding: 40px;
                                color: #e53e3e;
                            ">
                                {format!("Error: {}", error_msg)}
                            </div>
                        }
                    } else if filtered_items.is_empty() {
                        html! {
                            <div class="empty-state" style="
                                display: flex;
                                flex-direction: column;
                                align-items: center;
                                justify-content: center;
                                padding: 40px;
                                color: #666;
                            ">
                                <div style="font-size: 48px; margin-bottom: 16px;">{"📁"}</div>
                                <p style="margin: 0; text-align: center;">
                                    {if props.filter_images_only { 
                                        "No images found. Upload images in the Media Library first." 
                                    } else { 
                                        "No videos found. Upload video files in the Media Library first." 
                                    }}
                                </p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="media-grid" style="
                                display: grid;
                                grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
                                gap: 16px;
                            ">
                                {for filtered_items.iter().map(|item| {
                                    let item_clone = item.clone();
                                    let on_select = props.on_select.clone();
                                    let toggle_item_selection = toggle_item_selection.clone();
                                    let allow_multi_select = props.allow_multi_select;
                                    let is_selected = selected_items.iter().any(|selected| selected.id == item.id);
                                    
                                    html! {
                                        <div 
                                            key={item.id.unwrap_or(0)}
                                            class="media-item" 
                                            onclick={
                                                if allow_multi_select {
                                                    Callback::from(move |_| toggle_item_selection.emit(item_clone.clone()))
                                                } else {
                                                    Callback::from(move |_| on_select.emit(item_clone.clone()))
                                                }
                                            }
                                            style={format!("
                                                border: 2px solid {};
                                                border-radius: 8px;
                                                overflow: hidden;
                                                cursor: pointer;
                                                transition: all 0.2s ease;
                                                background: white;
                                                position: relative;
                                            ", if is_selected { "#007bff" } else { "#e1e5e9" })}

                                        >
                                            {if allow_multi_select {
                                                html! {
                                                    <div class="selection-indicator" style={format!("
                                                        position: absolute;
                                                        top: 8px;
                                                        right: 8px;
                                                        width: 24px;
                                                        height: 24px;
                                                        border-radius: 50%;
                                                        background: {};
                                                        border: 2px solid white;
                                                        display: flex;
                                                        align-items: center;
                                                        justify-content: center;
                                                        font-size: 12px;
                                                        font-weight: bold;
                                                        color: white;
                                                        z-index: 2;
                                                        box-shadow: 0 2px 4px rgba(0,0,0,0.2);
                                                    ", if is_selected { "#007bff" } else { "rgba(0,0,0,0.3)" })}>
                                                        {if is_selected { "✓" } else { "" }}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                            <div class="media-preview" style="
                                                aspect-ratio: 1;
                                                background: #f8f9fa;
                                                display: flex;
                                                align-items: center;
                                                justify-content: center;
                                                overflow: hidden;
                                            ">
                                                {if item.type_.starts_with("image") {
                                                    html! {
                                                        <img 
                                                            src={format!("http://localhost:8081{}", item.url)}
                                                            alt={item.name.clone()}
                                                            style="
                                                                width: 100%;
                                                                height: 100%;
                                                                object-fit: cover;
                                                            "
                                                            // Add loading attribute for better performance
                                                            loading="lazy"
                                                        />
                                                    }
                                                } else {
                                                    html! {
                                                        <div style="
                                                            font-size: 32px;
                                                            color: #666;
                                                        ">
                                                            {if item.type_.starts_with("video") { "🎥" }
                                                             else if item.type_.starts_with("audio") { "🎵" }
                                                             else if item.type_.contains("pdf") { "📄" }
                                                             else { "📁" }}
                                                        </div>
                                                    }
                                                }}
                                            </div>
                                            <div class="media-info" style="
                                                padding: 12px;
                                                border-top: 1px solid #e1e5e9;
                                            ">
                                                <div class="media-name" style="
                                                    font-size: 12px;
                                                    font-weight: 600;
                                                    color: #333;
                                                    margin-bottom: 4px;
                                                    white-space: nowrap;
                                                    overflow: hidden;
                                                    text-overflow: ellipsis;
                                                " title={item.name.clone()}>
                                                    {&item.name}
                                                </div>
                                                <div class="media-type" style="
                                                    font-size: 11px;
                                                    color: #666;
                                                ">
                                                    {&item.type_}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}