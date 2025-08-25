use yew::prelude::*;
use crate::services::api_service::{get_media, MediaItem};
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct MediaPickerProps {
    pub on_select: Callback<MediaItem>,
    pub on_close: Callback<()>,
    pub show: bool,
    pub filter_images_only: bool,
}

#[function_component(MediaPicker)]
pub fn media_picker(props: &MediaPickerProps) -> Html {
    let media_items = use_state(Vec::<MediaItem>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let search_term = use_state(|| String::new());

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
                        {if props.filter_images_only { "Select Image" } else { "Select Video" }}
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
                        "
                    />
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
                                    
                                    html! {
                                        <div 
                                            key={item.id.unwrap_or(0)}
                                            class="media-item" 
                                            onclick={Callback::from(move |_| on_select.emit(item_clone.clone()))}
                                            style="
                                                border: 1px solid #e1e5e9;
                                                border-radius: 8px;
                                                overflow: hidden;
                                                cursor: pointer;
                                                transition: all 0.2s ease;
                                                background: white;
                                            "

                                        >
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