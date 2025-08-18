use yew::prelude::*;
use crate::services::api_service::{get_media, delete_media, upload_media, MediaItem};
use web_sys::{File, HtmlInputElement, DragEvent, FileList, InputEvent, MouseEvent};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
enum ViewMode {
    Grid,
    List,
}

#[derive(Clone, PartialEq)]
enum MediaFilter {
    All,
    Images,
    Documents,
    Videos,
    Audio,
}

impl MediaFilter {
    fn matches(&self, media_type: &str) -> bool {
        match self {
            MediaFilter::All => true,
            MediaFilter::Images => media_type.starts_with("image"),
            MediaFilter::Documents => media_type.contains("pdf") || media_type.contains("text") || media_type.contains("document"),
            MediaFilter::Videos => media_type.starts_with("video"),
            MediaFilter::Audio => media_type.starts_with("audio"),
        }
    }

    #[allow(dead_code)]
    fn label(&self) -> &'static str {
        match self {
            MediaFilter::All => "All Media",
            MediaFilter::Images => "Images",
            MediaFilter::Documents => "Documents", 
            MediaFilter::Videos => "Videos",
            MediaFilter::Audio => "Audio",
        }
    }
}

async fn upload_file_with_logging(file: &File) -> Result<MediaItem, String> {
    use web_sys::console;
    
    console::log_1(&format!("🚀 Uploading: {} ({} bytes)", file.name(), file.size()).into());
    
    match upload_media(file).await {
        Ok(media_item) => {
            console::log_1(&"✅ Upload successful".into());
            Ok(media_item)
        }
        Err(e) => {
            console::log_1(&format!("❌ Upload failed: {}", e).into());
            Err(e.to_string())
        }
    }
}

#[function_component(MediaLibrary)]
pub fn media_library() -> Html {
    let media_items = use_state(Vec::<MediaItem>::new);
    let filtered_items = use_state(Vec::<MediaItem>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let uploading = use_state(|| false);
    let upload_progress = use_state(Vec::<(String, bool)>::new); // (filename, is_complete)
    let view_mode = use_state(|| ViewMode::Grid);
    let filter = use_state(|| MediaFilter::All);
    let search_term = use_state(|| String::new());
    let drag_over = use_state(|| false);
    let show_lightbox = use_state(|| false);
    let lightbox_image_url = use_state(String::new);
    let lightbox_image_name = use_state(String::new);

    // Load media on mount
    {
        let media_items = media_items.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
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
            || ()
        }, ());
    }

    // Update filtered items when media, filter, or search changes
    {
        let media_items_ref = media_items.clone();
        let filtered_items_ref = filtered_items.clone();
        let filter_ref = filter.clone();
        let search_term_ref = search_term.clone();

        use_effect_with_deps(move |(items, current_filter, current_search)| {
            let mut filtered: Vec<MediaItem> = items.iter()
                .filter(|item| current_filter.matches(&item.type_))
                .filter(|item| {
                    if current_search.is_empty() {
                        true
                    } else {
                        item.name.to_lowercase().contains(&current_search.to_lowercase())
                    }
                })
                .cloned()
                .collect();
            
            // Sort by creation date (newest first)
            filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            filtered_items_ref.set(filtered);
            || ()
        }, ((*media_items_ref).clone(), (*filter_ref).clone(), (*search_term_ref).clone()));
    }

    let handle_files = {
        let media_items = media_items.clone();
        let error = error.clone();
        let uploading = uploading.clone();
        let upload_progress = upload_progress.clone();
        Callback::from(move |files: FileList| {
            let media_items = media_items.clone();
            let error = error.clone();
            let uploading = uploading.clone();
            let upload_progress = upload_progress.clone();
            
            uploading.set(true);
            error.set(None);
            
            // Initialize progress tracking
            let mut progress = Vec::new();
            for i in 0..files.length() {
                if let Some(file) = files.get(i) {
                    progress.push((file.name(), false));
                }
            }
            upload_progress.set(progress);
            
            wasm_bindgen_futures::spawn_local(async move {
                let mut successful_uploads = Vec::new();
                let mut errors = Vec::new();
                
                for i in 0..files.length() {
                    if let Some(file) = files.get(i) {
                        match upload_file_with_logging(&file).await {
                            Ok(new_media) => {
                                successful_uploads.push(new_media);
                                
                                // Update progress
                                let mut current_progress = (*upload_progress).clone();
                                if let Some(item) = current_progress.iter_mut().find(|(name, _)| name == &file.name()) {
                                    item.1 = true;
                                }
                                upload_progress.set(current_progress);
                            }
                            Err(e) => {
                                errors.push(format!("{}: {}", file.name(), e));
                            }
                        }
                    }
                }
                
                // Update media list with successful uploads
                if !successful_uploads.is_empty() {
                    let mut current_media = (*media_items).clone();
                    current_media.extend(successful_uploads);
                    media_items.set(current_media);
                }
                
                // Set error if any uploads failed
                if !errors.is_empty() {
                    error.set(Some(errors.join("; ")));
                }
                
                uploading.set(false);
                // Clear progress after a delay
                gloo_timers::callback::Timeout::new(2000, move || {
                    upload_progress.set(Vec::new());
                }).forget();
            });
        })
    };

    let on_file_select = {
        let handle_files = handle_files.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            if let Some(files) = target.files() {
                handle_files.emit(files);
            }
        })
    };

    let on_drag_over = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(true);
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |_: DragEvent| {
            drag_over.set(false);
        })
    };

    let on_drop = {
        let drag_over = drag_over.clone();
        let _handle_files = handle_files.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(false);
            
            // Note: drag_drop data_transfer access needs different approach in WASM
            // For now, we'll handle file selection via the input element
        })
    };

    let on_delete_media = {
        let media_items = media_items.clone();
        let error = error.clone();
        Callback::from(move |media_id: i32| {
            let media_items = media_items.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_media(media_id).await {
                    Ok(_) => {
                        let mut current_media = (*media_items).clone();
                        current_media.retain(|item| item.id != Some(media_id));
                        media_items.set(current_media);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to delete: {}", e)));
                    }
                }
            });
        })
    };

    // Lightbox handlers
    let open_lightbox = {
        let show_lightbox = show_lightbox.clone();
        let lightbox_image_url = lightbox_image_url.clone();
        let lightbox_image_name = lightbox_image_name.clone();
        Callback::from(move |(url, name): (String, String)| {
            lightbox_image_url.set(format!("http://127.0.0.1:8081{}", url));
            lightbox_image_name.set(name);
            show_lightbox.set(true);
        })
    };

    let close_lightbox = {
        let show_lightbox = show_lightbox.clone();
        Callback::from(move |_| {
            show_lightbox.set(false);
        })
    };

    let on_search = {
        let search_term = search_term.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            search_term.set(target.value());
        })
    };

    let on_filter_change = {
        let filter = filter.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
            let new_filter = match target.value().as_str() {
                "images" => MediaFilter::Images,
                "documents" => MediaFilter::Documents,
                "videos" => MediaFilter::Videos,
                "audio" => MediaFilter::Audio,
                _ => MediaFilter::All,
            };
            filter.set(new_filter);
        })
    };

    let toggle_view = {
        let view_mode = view_mode.clone();
        Callback::from(move |new_mode: ViewMode| {
            view_mode.set(new_mode);
        })
    };

    html! {
        <div class="modern-media-library">
            // Header
            <div class="page-header">
                <div>
                    <h1>{"Media Library"}</h1>
                    <p>{"Manage your images, documents, and files"}</p>
                </div>
                <div class="header-actions">
                    <span class="stat">
                        <strong>{filtered_items.len()}</strong>
                        {" items"}
                    </span>
                </div>
            </div>

            // Upload Zone
            <div 
                class={classes!("upload-zone", if *drag_over { Some("drag-over") } else { None })}
                ondragover={on_drag_over}
                ondragleave={on_drag_leave}
                ondrop={on_drop}
            >
                {if *uploading {
                    html! {
                        <div class="upload-progress">
                            <div class="progress-header">
                                <h3>{"📤 Uploading Files..."}</h3>
                            </div>
                            <div class="progress-items">
                                {upload_progress.iter().map(|(filename, completed)| {
                                    html! {
                                        <div class={classes!("progress-item", if *completed { Some("completed") } else { None })}>
                                            <span class="filename">{filename}</span>
                                            <span class="status">
                                                {if *completed { "✅" } else { "⏳" }}
                                            </span>
                                        </div>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <>
                            <div class="upload-content">
                                <div class="upload-icon">{"☁️"}</div>
                                <h3>{"Drag & Drop Files Here"}</h3>
                                <p>{"or click to browse"}</p>
                                <label class="upload-button" for="file-upload">
                                    {"Choose Files"}
                                </label>
                                <input 
                                    type="file" 
                                    id="file-upload" 
                                    multiple=true
                                    style="display: none;" 
                                    onchange={on_file_select}
                                    accept="image/*,video/*,application/pdf,text/*,.doc,.docx,.xlsx,.zip"
                                />
                                <div class="supported-formats">
                                    {"Supports: Images, Videos, Documents, PDFs"}
                                </div>
                            </div>
                        </>
                    }
                }}
            </div>

            // Error Display
            {if let Some(ref error_msg) = *error {
                html! {
                    <div class="error-banner">
                        <span class="error-icon">{"⚠️"}</span>
                        <span class="error-text">{error_msg}</span>
                        <button class="error-close" onclick={Callback::from(move |_| error.set(None))}>{"×"}</button>
                    </div>
                }
            } else {
                html! {}
            }}

            // Controls
            <div class="media-controls">
                <div class="controls-left">
                    <select class="filter-select" onchange={on_filter_change}>
                        <option value="all" selected={matches!(*filter, MediaFilter::All)}>{"All Media"}</option>
                        <option value="images" selected={matches!(*filter, MediaFilter::Images)}>{"Images"}</option>
                        <option value="documents" selected={matches!(*filter, MediaFilter::Documents)}>{"Documents"}</option>
                        <option value="videos" selected={matches!(*filter, MediaFilter::Videos)}>{"Videos"}</option>
                        <option value="audio" selected={matches!(*filter, MediaFilter::Audio)}>{"Audio"}</option>
                    </select>
                    <input 
                        type="text" 
                        placeholder="🔍 Search media..." 
                        class="search-input" 
                        value={(*search_term).clone()}
                        oninput={on_search}
                    />
                </div>
                <div class="controls-right">
                    <div class="view-toggle">
                        <button 
                            class={classes!("view-btn", if matches!(*view_mode, ViewMode::Grid) { Some("active") } else { None })}
                            onclick={let toggle = toggle_view.clone(); Callback::from(move |_| toggle.emit(ViewMode::Grid))}
                        >
                            {"⊞"}
                        </button>
                        <button 
                            class={classes!("view-btn", if matches!(*view_mode, ViewMode::List) { Some("active") } else { None })}
                            onclick={let toggle = toggle_view.clone(); Callback::from(move |_| toggle.emit(ViewMode::List))}
                        >
                            {"☰"}
                        </button>
                    </div>
                </div>
            </div>

            // Content Area
            <div class="media-content">
                {if *loading {
                    html! {
                        <div class="loading-state">
                            <div class="loader"></div>
                            <p>{"Loading media..."}</p>
                        </div>
                    }
                } else if filtered_items.is_empty() {
                    html! {
                        <div class="empty-state">
                            <div class="empty-icon">{"📁"}</div>
                            <h3>{"No media found"}</h3>
                            <p>
                                {if media_items.is_empty() {
                                    "Upload your first media file to get started!"
                                } else {
                                    "Try adjusting your search or filter."
                                }}
                            </p>
                        </div>
                    }
                } else {
                    html! {
                        <div class={classes!("media-grid", if matches!(*view_mode, ViewMode::List) { Some("list-view") } else { None })}>
                            {filtered_items.iter().map(|item| {
                                let on_delete = {
                                    let on_delete_media = on_delete_media.clone();
                                    let item_id = item.id.unwrap_or(0);
                                    Callback::from(move |_| on_delete_media.emit(item_id))
                                };

                                let on_view = {
                                    let open_lightbox = open_lightbox.clone();
                                    let url = item.url.clone();
                                    let name = item.name.clone();
                                    Callback::from(move |_| {
                                        if url.contains("image") || url.ends_with(".jpg") || url.ends_with(".png") || url.ends_with(".gif") || url.ends_with(".jpeg") {
                                            open_lightbox.emit((url.clone(), name.clone()));
                                        }
                                    })
                                };

                                let (media_icon, media_class) = get_media_icon_and_class(&item.type_);
                                let file_extension = item.name.split('.').last().unwrap_or("").to_uppercase();

                                html! {
                                    <div class={classes!("media-card", Some(media_class))}>
                                        <div class="media-preview">
                                            {if item.type_.starts_with("image") && !item.url.is_empty() {
                                                html! { <img src={format!("http://127.0.0.1:8081{}", item.url)} alt={item.name.clone()} /> }
                                            } else {
                                                html! {
                                                    <div class="file-icon">
                                                        <span class="icon">{media_icon}</span>
                                                        <span class="extension">{file_extension}</span>
                                                    </div>
                                                }
                                            }}
                                            <div class="media-overlay">
                                                <button class="action-btn view-btn" title="View" onclick={on_view}>{"👁"}</button>
                                                <button class="action-btn delete-btn" title="Delete" onclick={on_delete}>{"🗑"}</button>
                                            </div>
                                        </div>
                                        <div class="media-info">
                                            <h4 class="media-name" title={item.name.clone()}>{&item.name}</h4>
                                            <div class="media-meta">
                                                <span class="media-type">{&item.type_}</span>
                                                {if let Some(ref size) = item.size {
                                                    html! { <span class="media-size">{size}</span> }
                                                } else {
                                                    html! {}
                                                }}
                                                {if let Some(ref created) = item.created_at {
                                                    html! { <span class="media-date">{created}</span> }
                                                } else {
                                                    html! {}
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()}
                        </div>
                    }
                }}
            </div>

            // Lightbox Modal
            {if *show_lightbox {
                html! {
                    <div class="lightbox-overlay" onclick={close_lightbox.clone()}>
                        <div class="lightbox-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                            <button class="lightbox-close" onclick={close_lightbox.clone()}>{"×"}</button>
                            <img src={(*lightbox_image_url).clone()} alt={(*lightbox_image_name).clone()} />
                            <div class="lightbox-caption">{(*lightbox_image_name).clone()}</div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn get_media_icon_and_class(media_type: &str) -> (&'static str, &'static str) {
    if media_type.starts_with("image") {
        ("🖼️", "image")
    } else if media_type.starts_with("video") {
        ("🎥", "video")
    } else if media_type.starts_with("audio") {
        ("🎵", "audio")
    } else if media_type.contains("pdf") {
        ("📄", "pdf")
    } else if media_type.contains("text") || media_type.contains("document") {
        ("📝", "document")
    } else if media_type.contains("zip") || media_type.contains("archive") {
        ("📦", "archive")
    } else {
        ("📁", "file")
    }
}