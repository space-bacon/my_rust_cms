use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::services::api_service::MediaItem;
// MediaPicker is handled by parent component
use web_sys::{MouseEvent, TouchEvent, KeyboardEvent, DragEvent};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhancedGalleryImage {
    pub id: String,
    pub url: String,
    pub alt: String,
    pub caption: String,
    pub title: String,
    pub media_id: Option<i32>,
}

impl Default for EnhancedGalleryImage {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url: String::new(),
            alt: String::new(),
            caption: String::new(),
            title: String::new(),
            media_id: None,
        }
    }
}

impl From<MediaItem> for EnhancedGalleryImage {
    fn from(media: MediaItem) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url: format!("http://localhost:8081{}", media.url),
            alt: media.name.clone(),
            caption: String::new(),
            title: media.name,
            media_id: media.id,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct EnhancedGalleryProps {
    pub images: Vec<EnhancedGalleryImage>,
    pub on_images_change: Callback<Vec<EnhancedGalleryImage>>,
    pub layout: String, // "grid", "masonry", "carousel", "slider"
    pub columns: i32,
    pub gap: i32,
    pub border_radius: i32,
    pub show_captions: bool,
    pub enable_lightbox: bool,
    pub enable_drag_reorder: bool,
    #[prop_or(false)]
    pub is_admin_mode: bool,
}

#[function_component(EnhancedGallery)]
pub fn enhanced_gallery(props: &EnhancedGalleryProps) -> Html {
    let current_image_index = use_state(|| 0usize);
    let show_lightbox = use_state(|| false);
    // Media picker state is handled by parent component
    let show_caption_modal = use_state(|| false);
    let caption_modal_image_id = use_state(|| String::new());
    let drag_over_index = use_state(|| None::<usize>);
    let dragging_index = use_state(|| None::<usize>);
    
    // Touch/swipe handling
    let touch_start_x = use_state(|| 0f64);
    let touch_start_y = use_state(|| 0f64);
    let is_swiping = use_state(|| false);

    // Media management is handled by the page builder properties panel

    // Media selection is handled by parent component

    let on_remove_image = {
        let images = props.images.clone();
        let on_images_change = props.on_images_change.clone();
        
        Callback::from(move |index: usize| {
            let mut new_images = images.clone();
            if index < new_images.len() {
                new_images.remove(index);
                on_images_change.emit(new_images);
            }
        })
    };

    let on_thumbnail_click = {
        let current_image_index = current_image_index.clone();
        let show_lightbox = show_lightbox.clone();
        let enable_lightbox = props.enable_lightbox;
        let is_admin_mode = props.is_admin_mode;
        
        Callback::from(move |index: usize| {
            current_image_index.set(index);
            // In admin mode, thumbnail clicks open lightbox
            // In public mode, thumbnail clicks just change the main image
            if is_admin_mode && enable_lightbox {
                show_lightbox.set(true);
            }
        })
    };

    let on_info_click = {
        let show_caption_modal = show_caption_modal.clone();
        let caption_modal_image_id = caption_modal_image_id.clone();
        
        Callback::from(move |image_id: String| {
            caption_modal_image_id.set(image_id);
            show_caption_modal.set(true);
        })
    };

    let on_caption_update = {
        let images = props.images.clone();
        let on_images_change = props.on_images_change.clone();
        let show_caption_modal = show_caption_modal.clone();
        let caption_modal_image_id = caption_modal_image_id.clone();
        
        Callback::from(move |caption: String| {
            let image_id = (*caption_modal_image_id).clone();
            let mut new_images = images.clone();
            if let Some(image) = new_images.iter_mut().find(|img| img.id == image_id) {
                image.caption = caption;
                on_images_change.emit(new_images);
            }
            show_caption_modal.set(false);
        })
    };

    // Lightbox navigation
    let navigate_lightbox = {
        let current_image_index = current_image_index.clone();
        let images_len = props.images.len();
        
        Callback::from(move |direction: i32| {
            if images_len > 0 {
                let current = *current_image_index;
                let new_index = if direction > 0 {
                    (current + 1) % images_len
                } else {
                    if current == 0 { images_len - 1 } else { current - 1 }
                };
                current_image_index.set(new_index);
            }
        })
    };

    // Keyboard navigation for lightbox
    let on_lightbox_keydown = {
        let navigate_lightbox = navigate_lightbox.clone();
        let show_lightbox = show_lightbox.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowLeft" => {
                    e.prevent_default();
                    navigate_lightbox.emit(-1);
                }
                "ArrowRight" => {
                    e.prevent_default();
                    navigate_lightbox.emit(1);
                }
                "Escape" => {
                    e.prevent_default();
                    show_lightbox.set(false);
                }
                _ => {}
            }
        })
    };

    // Touch/swipe handling for lightbox
    let on_lightbox_touchstart = {
        let _touch_start_x = touch_start_x.clone();
        let _touch_start_y = touch_start_y.clone();
        let is_swiping = is_swiping.clone();
        
        Callback::from(move |_e: TouchEvent| {
            // Simplified touch handling - just enable swiping
            is_swiping.set(true);
        })
    };

    let on_lightbox_touchend = {
        let _touch_start_x = touch_start_x.clone();
        let navigate_lightbox = navigate_lightbox.clone();
        let is_swiping = is_swiping.clone();
        
        Callback::from(move |_e: TouchEvent| {
            if *is_swiping {
                // Simplified: just navigate to next on touch end
                navigate_lightbox.emit(1);
                is_swiping.set(false);
            }
        })
    };

    // Drag and drop for reordering
    let on_drag_start = {
        let dragging_index = dragging_index.clone();
        
        Callback::from(move |e: DragEvent| {
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                    if let Some(index_str) = element.get_attribute("data-index") {
                        if let Ok(index) = index_str.parse::<usize>() {
                            dragging_index.set(Some(index));
                            // Simplified drag handling without data transfer
                        }
                    }
                }
            }
        })
    };

    let on_drag_over = {
        let drag_over_index = drag_over_index.clone();
        
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                    if let Some(index_str) = element.get_attribute("data-index") {
                        if let Ok(index) = index_str.parse::<usize>() {
                            drag_over_index.set(Some(index));
                        }
                    }
                }
            }
        })
    };

    let on_drop = {
        let images = props.images.clone();
        let on_images_change = props.on_images_change.clone();
        let dragging_index = dragging_index.clone();
        let drag_over_index = drag_over_index.clone();
        
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            
            if let (Some(from_index), Some(to_index)) = (*dragging_index, *drag_over_index) {
                if from_index != to_index {
                    let mut new_images = images.clone();
                    let item = new_images.remove(from_index);
                    new_images.insert(to_index, item);
                    on_images_change.emit(new_images);
                }
            }
            
            dragging_index.set(None);
            drag_over_index.set(None);
        })
    };

    html! {
        <div class="enhanced-gallery-container">
            <style>
                {include_str!("../styles/enhanced_gallery.css")}
            </style>
            
            <div class="enhanced-gallery" style={format!(
                "--gallery-columns: {}; --gallery-gap: {}px; --gallery-border-radius: {}px;",
                props.columns, props.gap, props.border_radius
            )}>
                
                // Thumbnail sidebar
                <div class="gallery-thumbnails">
                    <div class="thumbnails-header">
                        <h4>{"Gallery Items"}</h4>
                    </div>
                    
                    <div class="thumbnails-list">
                        {if props.images.is_empty() {
                            html! {
                                <div class="empty-thumbnails">
                                    <div class="empty-icon">{"🖼️"}</div>
                                    <p>{"No images in gallery"}</p>
                                </div>
                            }
                        } else {
                            html! {
                                <>
                                    {for props.images.iter().enumerate().map(|(index, image)| {
                                        let on_thumbnail_click = on_thumbnail_click.clone();
                                        let on_remove_image = on_remove_image.clone();
                                        let on_info_click = on_info_click.clone();
                                        let image_id = image.id.clone();
                                        let is_current = index == *current_image_index;
                                        
                                        html! {
                                            <div 
                                                key={image.id.clone()}
                                                class={classes!("thumbnail-item", if is_current { Some("active") } else { None })}
                                                data-index={index.to_string()}
                                                draggable={props.enable_drag_reorder.to_string()}
                                                ondragstart={if props.enable_drag_reorder { Some(on_drag_start.clone()) } else { None }}
                                                ondragover={if props.enable_drag_reorder { Some(on_drag_over.clone()) } else { None }}
                                                ondrop={if props.enable_drag_reorder { Some(on_drop.clone()) } else { None }}
                                            >
                                                <div 
                                                    class="thumbnail-preview"
                                                    onclick={let on_thumbnail_click = on_thumbnail_click.clone(); let index = index; Callback::from(move |_| on_thumbnail_click.emit(index))}
                                                >
                                                    <img 
                                                        src={image.url.clone()}
                                                        alt={image.alt.clone()}
                                                        loading="lazy"
                                                    />
                                                    {if props.enable_drag_reorder {
                                                        html! {
                                                            <div class="drag-handle" title="Drag to reorder">
                                                                {"⋮⋮"}
                                                            </div>
                                                        }
                                                    } else { html! {} }}
                                                </div>
                                                
                                                {if props.is_admin_mode {
                                                    html! {
                                                        <div class="thumbnail-actions">
                                                            <button 
                                                                class="action-btn view-btn"
                                                                onclick={let on_thumbnail_click = on_thumbnail_click.clone(); let index = index; Callback::from(move |_| on_thumbnail_click.emit(index))}
                                                                title="View Details"
                                                            >
                                                                {"👁️"}
                                                            </button>
                                                            <button 
                                                                class="action-btn info-btn"
                                                                onclick={let image_id = image_id.clone(); Callback::from(move |_| on_info_click.emit(image_id.clone()))}
                                                                title="Edit Caption"
                                                            >
                                                                {"ℹ️"}
                                                            </button>
                                                            <button 
                                                                class="action-btn remove-btn"
                                                                onclick={let index = index; Callback::from(move |_| on_remove_image.emit(index))}
                                                                title="Remove Image"
                                                            >
                                                                {"🗑️"}
                                                            </button>
                                                        </div>
                                                    }
                                                } else { html! {} }}
                                                
                                                {if !image.caption.is_empty() {
                                                    html! {
                                                        <div class="thumbnail-caption" title={image.caption.clone()}>
                                                            {&image.caption}
                                                        </div>
                                                    }
                                                } else { html! {} }}
                                            </div>
                                        }
                                    })}
                                </>
                            }
                        }}
                    </div>
                </div>
                
                // Main viewport
                <div class="gallery-viewport">
                    {if !props.images.is_empty() {
                        let current_image = &props.images[*current_image_index];
                        html! {
                            <div class="viewport-content">
                                <div class="main-image-container">
                                    <img 
                                        src={current_image.url.clone()}
                                        alt={current_image.alt.clone()}
                                        class="main-image"
                                        onclick={if props.enable_lightbox { 
                                            let show_lightbox = show_lightbox.clone();
                                            Some(Callback::from(move |_| show_lightbox.set(true)))
                                        } else { None }}
                                    />
                                    
                                    {if props.show_captions && !current_image.caption.is_empty() {
                                        html! {
                                            <div class="main-image-caption">
                                                {&current_image.caption}
                                            </div>
                                        }
                                    } else { html! {} }}
                                    
                                    // Navigation arrows
                                    {if props.images.len() > 1 {
                                        html! {
                                            <>
                                                <button 
                                                    class="nav-arrow nav-prev"
                                                    onclick={let navigate_lightbox = navigate_lightbox.clone(); Callback::from(move |_| navigate_lightbox.emit(-1))}
                                                    title="Previous Image"
                                                >
                                                    {"‹"}
                                                </button>
                                                <button 
                                                    class="nav-arrow nav-next"
                                                    onclick={let navigate_lightbox = navigate_lightbox.clone(); Callback::from(move |_| navigate_lightbox.emit(1))}
                                                    title="Next Image"
                                                >
                                                    {"›"}
                                                </button>
                                            </>
                                        }
                                    } else { html! {} }}
                                </div>
                                
                                // Image info
                                <div class="image-info">
                                    <div class="image-title">{&current_image.title}</div>
                                    <div class="image-counter">
                                        {format!("{} of {}", *current_image_index + 1, props.images.len())}
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="empty-viewport">
                                <div class="empty-icon">{"🖼️"}</div>
                                <h3>{"Gallery Empty"}</h3>
                                <p>{"No images to display"}</p>
                            </div>
                        }
                    }}
                </div>
            </div>
            
            // Media picker is handled by parent component
            
            // Lightbox Modal
            {if *show_lightbox && !props.images.is_empty() {
                let current_image = &props.images[*current_image_index];
                html! {
                    <div 
                        class="lightbox-overlay"
                        onclick={let show_lightbox = show_lightbox.clone(); Callback::from(move |_| show_lightbox.set(false))}
                        onkeydown={on_lightbox_keydown}
                        ontouchstart={on_lightbox_touchstart}
                        ontouchend={on_lightbox_touchend}
                        tabindex="0"
                    >
                        <div class="lightbox-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                            <button 
                                class="lightbox-close"
                                onclick={let show_lightbox = show_lightbox.clone(); Callback::from(move |_| show_lightbox.set(false))}
                            >
                                {"×"}
                            </button>
                            
                            <img 
                                src={current_image.url.clone()}
                                alt={current_image.alt.clone()}
                                class="lightbox-image"
                            />
                            
                            {if props.images.len() > 1 {
                                html! {
                                    <>
                                        <button 
                                            class="lightbox-nav lightbox-prev"
                                            onclick={
                                                let navigate_lightbox = navigate_lightbox.clone(); 
                                                Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation(); // Prevent modal close
                                                    navigate_lightbox.emit(-1);
                                                })
                                            }
                                            title="Previous Image"
                                        >
                                            <span class="nav-icon">{"‹"}</span>
                                        </button>
                                        <button 
                                            class="lightbox-nav lightbox-next"
                                            onclick={
                                                let navigate_lightbox = navigate_lightbox.clone(); 
                                                Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation(); // Prevent modal close
                                                    navigate_lightbox.emit(1);
                                                })
                                            }
                                            title="Next Image"
                                        >
                                            <span class="nav-icon">{"›"}</span>
                                        </button>
                                        
                                        // Image counter
                                        <div class="lightbox-counter">
                                            {format!("{} / {}", *current_image_index + 1, props.images.len())}
                                        </div>
                                    </>
                                }
                            } else { html! {} }}
                            
                            {if !current_image.caption.is_empty() {
                                html! {
                                    <div class="lightbox-caption">
                                        {&current_image.caption}
                                    </div>
                                }
                            } else { html! {} }}
                        </div>
                    </div>
                }
            } else { html! {} }}
            
            // Caption Modal
            {if *show_caption_modal {
                let current_caption = props.images.iter()
                    .find(|img| img.id == *caption_modal_image_id)
                    .map(|img| img.caption.clone())
                    .unwrap_or_default();
                    
                html! {
                    <CaptionModal 
                        show={*show_caption_modal}
                        current_caption={current_caption}
                        on_save={on_caption_update}
                        on_close={let show_caption_modal = show_caption_modal.clone(); Callback::from(move |_| show_caption_modal.set(false))}
                    />
                }
            } else { html! {} }}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CaptionModalProps {
    pub show: bool,
    pub current_caption: String,
    pub on_save: Callback<String>,
    pub on_close: Callback<()>,
}

#[function_component(CaptionModal)]
pub fn caption_modal(props: &CaptionModalProps) -> Html {
    let caption_input = use_state(|| props.current_caption.clone());
    
    let on_input = {
        let caption_input = caption_input.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
            caption_input.set(target.value());
        })
    };
    
    let on_save = {
        let caption_input = caption_input.clone();
        let on_save = props.on_save.clone();
        Callback::from(move |_| {
            on_save.emit((*caption_input).clone());
        })
    };
    
    if !props.show {
        return html! {};
    }
    
    html! {
        <div class="caption-modal-overlay" onclick={let on_close = props.on_close.clone(); Callback::from(move |_| on_close.emit(()))}>
            <div class="caption-modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class="caption-modal-header">
                    <h3>{"Edit Image Caption"}</h3>
                    <button class="close-btn" onclick={let on_close = props.on_close.clone(); Callback::from(move |_| on_close.emit(()))}>{"×"}</button>
                </div>
                
                <div class="caption-modal-body">
                    <label for="caption-input">{"Caption:"}</label>
                    <textarea 
                        id="caption-input"
                        value={(*caption_input).clone()}
                        oninput={on_input}
                        placeholder="Enter a caption for this image..."
                        rows="4"
                    />
                </div>
                
                <div class="caption-modal-footer">
                    <button class="cancel-btn" onclick={let on_close = props.on_close.clone(); Callback::from(move |_| on_close.emit(()))}>
                        {"Cancel"}
                    </button>
                    <button class="save-btn" onclick={on_save}>
                        {"Save Caption"}
                    </button>
                </div>
            </div>
        </div>
    }
}
