use yew::prelude::*;
use crate::services::api_service::{get_posts, get_media, get_comments, get_pages, Post, MediaItem};
use crate::services::migrate_pages::create_essential_pages;
use crate::components::admin::sidebar::AdminTab;

#[derive(Properties, PartialEq)]
pub struct AdminDashboardProps {
    pub on_navigate: Callback<AdminTab>,
}

#[derive(Clone, PartialEq)]
pub struct DashboardStats {
    pub total_posts: i32,
    pub published_posts: i32,
    pub draft_posts: i32,
    pub total_media: i32,
    pub total_comments: i32,
    pub pending_comments: i32,
}

#[function_component(AdminDashboard)]
pub fn admin_dashboard(props: &AdminDashboardProps) -> Html {

    let stats = use_state(|| DashboardStats {
        total_posts: 0,
        published_posts: 0,
        draft_posts: 0,
        total_media: 0,
        total_comments: 0,
        pending_comments: 0,
    });
    
    let recent_posts = use_state(Vec::<Post>::new);
    let recent_media = use_state(Vec::<MediaItem>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let show_create_essentials = use_state(|| false);

    // Load dashboard data and ensure essential pages button visibility
    {
        let stats = stats.clone();
        let recent_posts = recent_posts.clone();
        let recent_media = recent_media.clone();
        let loading = loading.clone();
        let error = error.clone();
        let show_create_essentials_handle = show_create_essentials.clone();
        
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);
                // Check if essential pages exist
                match get_pages().await {
                    Ok(pages) => {
                        let has_home = pages.iter().any(|p| p.slug == "home");
                        let has_posts = pages.iter().any(|p| p.slug == "posts");
                        let has_why = pages.iter().any(|p| p.slug == "why-my-rust-cms");
                        show_create_essentials_handle.set(!(has_home && has_posts && has_why));
                    }
                    Err(_) => {}
                }
                
                // Load posts
                match get_posts().await {
                    Ok(posts) => {
                        let total_posts = posts.len() as i32;
                        let published_posts = posts.iter().filter(|p| p.status == "published").count() as i32;
                        let draft_posts = posts.iter().filter(|p| p.status == "draft").count() as i32;
                        
                        // Get recent posts (last 5)
                        let mut recent = posts.clone();
                        recent.sort_by(|a, b| {
                            b.created_at.as_ref().unwrap_or(&"".to_string())
                                .cmp(a.created_at.as_ref().unwrap_or(&"".to_string()))
                        });
                        recent.truncate(5);
                        
                        recent_posts.set(recent);
                        
                        // Load media
                        match get_media().await {
                            Ok(media) => {
                                let total_media = media.len() as i32;
                                
                                // Get recent media (last 5)
                                let mut recent_media_items = media.clone();
                                recent_media_items.sort_by(|a, b| {
                                    b.created_at.as_ref().unwrap_or(&"".to_string())
                                        .cmp(a.created_at.as_ref().unwrap_or(&"".to_string()))
                                });
                                recent_media_items.truncate(5);
                                
                                recent_media.set(recent_media_items);
                                
                                // Load comments
                                match get_comments().await {
                                    Ok(comments) => {
                                        let total_comments = comments.len() as i32;
                                        let pending_comments = comments.iter()
                                            .filter(|c| c.status.to_lowercase() == "pending")
                                            .count() as i32;
                                        
                                        // Update stats with actual comment data
                                        stats.set(DashboardStats {
                                            total_posts,
                                            published_posts,
                                            draft_posts,
                                            total_media,
                                            total_comments,
                                            pending_comments,
                                        });
                                    }
                                    Err(e) => {
                                        // If comments fail to load, use 0 as fallback but log the error
                                        log::warn!("Failed to load comments: {}", e);
                                        stats.set(DashboardStats {
                                            total_posts,
                                            published_posts,
                                            draft_posts,
                                            total_media,
                                            total_comments: 0,
                                            pending_comments: 0,
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to load media: {}", e)));
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load posts: {}", e)));
                    }
                }
                
                loading.set(false);
            });
            || ()
        }, ());
    }



    html! {
        <div class="dashboard">
            <div class="page-header">
                <div>
                    <h1>{"Dashboard"}</h1>
                    <p>{"Welcome to your CMS dashboard. Here's an overview of your content."}</p>
                </div>
            </div>

            if *loading {
                <div class="loading">
                    <div class="loading-spinner"></div>
                    <p>{"Loading dashboard data..."}</p>
                </div>
            } else if let Some(error_msg) = &*error {
                <div class="error-message">
                    <h3>{"Error Loading Dashboard"}</h3>
                    <p>{error_msg}</p>
                </div>
            } else {
                <>
                    <div class="stats-grid">
                        <div class="stat-card">
                            <div class="stat-icon">{"📝"}</div>
                            <div class="stat-content">
                                <h3>{"Total Posts"}</h3>
                                <p class="stat-number">{stats.total_posts}</p>
                            </div>
                        </div>
                        
                        <div class="stat-card">
                            <div class="stat-icon">{"✅"}</div>
                            <div class="stat-content">
                                <h3>{"Published"}</h3>
                                <p class="stat-number">{stats.published_posts}</p>
                            </div>
                        </div>
                        
                        <div class="stat-card">
                            <div class="stat-icon">{"📝"}</div>
                            <div class="stat-content">
                                <h3>{"Drafts"}</h3>
                                <p class="stat-number">{stats.draft_posts}</p>
                            </div>
                        </div>
                        
                        <div class="stat-card">
                            <div class="stat-icon">{"🖼️"}</div>
                            <div class="stat-content">
                                <h3>{"Media Files"}</h3>
                                <p class="stat-number">{stats.total_media}</p>
                            </div>
                        </div>
                        
                        <div class="stat-card">
                            <div class="stat-icon">{"💬"}</div>
                            <div class="stat-content">
                                <h3>{"Comments"}</h3>
                                <p class="stat-number">{stats.total_comments}</p>
                            </div>
                        </div>
                        
                        <div class="stat-card">
                            <div class="stat-icon">{"⏳"}</div>
                            <div class="stat-content">
                                <h3>{"Pending"}</h3>
                                <p class="stat-number">{stats.pending_comments}</p>
                            </div>
                        </div>
                    </div>

                    <div class="dashboard-content">
                        <div class="recent-posts">
                            <h3>{"Recent Posts"}</h3>
                            if recent_posts.is_empty() {
                                <div class="empty-state">
                                    <p>{"No posts yet. Create your first post to get started!"}</p>
                                </div>
                            } else {
                                <div class="posts-list">
                                    {recent_posts.iter().map(|post| {
                                        html! {
                                            <div class="post-item" key={post.id.as_ref().map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_string())}>
                                                <div class="post-info">
                                                    <h4>{&post.title}</h4>
                                                    <div class="post-meta">
                                                        <span class="status-badge status-badge-{post.status.clone()}">{&post.status}</span>
                                                        <span class="post-date">
                                                            {post.created_at.as_ref().unwrap_or(&"Unknown date".to_string())}
                                                        </span>
                                                    </div>
                                                </div>
                                                <div class="post-actions">
                                                    <button class="btn btn-small">{"Edit"}</button>
                                                    <button class="btn btn-small btn-secondary">{"View"}</button>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            }
                        </div>

                        <div class="recent-media">
                            <h3>{"Recent Media"}</h3>
                            if recent_media.is_empty() {
                                <div class="empty-state">
                                    <p>{"No media files yet. Upload some files to get started!"}</p>
                                </div>
                            } else {
                                <div class="media-list">
                                    {recent_media.iter().map(|media| {
                                        html! {
                                                                                         <div class="media-item" key={media.id.as_ref().map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_string())}>
                                                <div class="media-preview">
                                                    {if media.type_.starts_with("image/") {
                                                        html! { <img src={format!("http://localhost:8081{}", media.url)} alt={media.name.clone()} /> }
                                                    } else {
                                                        html! { <div class="file-icon">{"📄"}</div> }
                                                    }}
                                                </div>
                                                <div class="media-info">
                                                    <h4>{&media.name}</h4>
                                                    <p class="media-meta">
                                                        {if let Some(ref size) = media.size {
                                                            html! { <span class="media-size">{size}</span> }
                                                        } else {
                                                            html! {}
                                                        }}
                                                        <span class="media-date">
                                                            {media.created_at.as_ref().unwrap_or(&"Unknown date".to_string())}
                                                        </span>
                                                    </p>
                                                </div>
                                                <div class="media-actions">
                                                    <button class="btn btn-small">{"Copy URL"}</button>
                                                    <button class="btn btn-small btn-secondary">{"Delete"}</button>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            }
                        </div>
                    </div>

                    <div class="quick-actions">
                        <h3>{"Quick Actions"}</h3>
                        <div class="action-buttons">
                            <button 
                                class="btn"
                                onclick={{
                                    let on_navigate = props.on_navigate.clone();
                                    Callback::from(move |_| on_navigate.emit(AdminTab::Posts))
                                }}
                            >{"Create New Post"}</button>
                            { if *show_create_essentials {
                                html! {
                                    <button class="btn btn-accent" onclick={Callback::from(move |_| {
                                        wasm_bindgen_futures::spawn_local(async move { let _ = create_essential_pages().await; });
                                    })}>{"Create Essential Pages"}</button>
                                }
                              } else { html!{} } }
                            <button 
                                class="btn btn-secondary"
                                onclick={{
                                    let on_navigate = props.on_navigate.clone();
                                    Callback::from(move |_| on_navigate.emit(AdminTab::Media))
                                }}
                            >{"Upload Media"}</button>
                            <button 
                                class="btn btn-secondary"
                                onclick={{
                                    let on_navigate = props.on_navigate.clone();
                                    Callback::from(move |_| on_navigate.emit(AdminTab::Comments))
                                }}
                            >{"Manage Comments"}</button>
                            <button 
                                class="btn btn-secondary"
                                onclick={{
                                    let on_navigate = props.on_navigate.clone();
                                    Callback::from(move |_| on_navigate.emit(AdminTab::Analytics))
                                }}
                            >{"View Analytics"}</button>
                        </div>
                    </div>
                </>
            }
        </div>
    }
} 