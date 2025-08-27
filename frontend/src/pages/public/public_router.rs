use yew::prelude::*;
use crate::components::{PublicLayout, PostsListWidget, CommentsSection, EnhancedGallery, EnhancedGalleryImage};
use crate::services::page_service::{get_page_by_slug, Page};
use crate::components::page_builder::{PageComponent, ComponentType};
use crate::services::default_pages::{get_default_home_page_components, get_default_posts_page_components};
use crate::services::navigation_service::check_comments_enabled;

#[derive(Clone, PartialEq, Debug)]
pub enum PublicPage {
    Home,
    Posts,
    Post(i32),
    Page(String),
}

#[derive(Properties, PartialEq)]
pub struct PublicRouterProps {
    pub current_page: PublicPage,
    pub on_admin_click: Callback<()>,
    pub on_navigate: Callback<PublicPage>,
}

#[function_component(PublicRouter)]
pub fn public_router(props: &PublicRouterProps) -> Html {
    let current_page_name = match &props.current_page {
        PublicPage::Home => "home",
        PublicPage::Posts => "posts",
        PublicPage::Post(_) => "post",
        PublicPage::Page(slug) => slug,
    };



    let content = match &props.current_page {
        PublicPage::Home => html! {
            <HomeContent 
                on_admin_click={props.on_admin_click.clone()} 
                on_navigate={props.on_navigate.clone()}
            />
        },
        PublicPage::Posts => html! {
            <PostsContent 
                on_admin_click={props.on_admin_click.clone()} 
                on_navigate={props.on_navigate.clone()}
            />
        },
        PublicPage::Post(id) => html! {
            <PostContent post_id={*id} on_admin_click={props.on_admin_click.clone()} />
        },
        PublicPage::Page(slug) => html! {
            <PageContent slug={slug.clone()} on_admin_click={props.on_admin_click.clone()} />
        },
    };

    html! {
        <PublicLayout
            on_admin_click={props.on_admin_click.clone()}
            on_navigate={Some(props.on_navigate.clone())}
            current_page={current_page_name.to_string()}
        >
            {content}
        </PublicLayout>
    }
}

#[derive(Properties, PartialEq)]
struct HomeContentProps {
    on_admin_click: Callback<()>,
    on_navigate: Callback<PublicPage>,
}

#[function_component(HomeContent)]
fn home_content(_props: &HomeContentProps) -> Html {
    let page = use_state(|| None::<Page>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let page = page.clone();
        let loading = loading.clone();
        let _error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_page_by_slug("home").await {
                    Ok(fetched_page) => {
                        page.set(Some(fetched_page));
                        loading.set(false);
                    }
                    Err(_) => {
                        // Page doesn't exist, use default components
                        let default_components = get_default_home_page_components();
                        let default_content = serde_json::to_string(&default_components).unwrap_or_default();
                        let default_page = Page {
                            id: None,
                            title: "Home".to_string(),
                            slug: "home".to_string(),
                            content: default_content,
                            status: "published".to_string(),
                            created_at: None,
                            updated_at: None,
                        };
                        page.set(Some(default_page));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="home-content">
            if *loading {
                <div class="loading">{"Loading page..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading page: "}{error_msg}</div>
            } else if let Some(ref page_data) = *page {
                <div class="page-content">
                    {render_page_builder_content_with_navigation_and_context(&page_data.content, None, page_data.id)}
                </div>
            } else {
                <div class="error">{"Page not found"}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PostsContentProps {
    on_admin_click: Callback<()>,
    on_navigate: Callback<PublicPage>,
}

#[function_component(PostsContent)]
fn posts_content(props: &PostsContentProps) -> Html {
    let page = use_state(|| None::<Page>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let page = page.clone();
        let loading = loading.clone();
        let _error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_page_by_slug("posts").await {
                    Ok(fetched_page) => {
                        page.set(Some(fetched_page));
                        loading.set(false);
                    }
                    Err(_) => {
                        // Page doesn't exist, use default components
                        let default_components = get_default_posts_page_components();
                        let default_content = serde_json::to_string(&default_components).unwrap_or_default();
                        let default_page = Page {
                            id: None,
                            title: "All Posts".to_string(),
                            slug: "posts".to_string(),
                            content: default_content,
                            status: "published".to_string(),
                            created_at: None,
                            updated_at: None,
                        };
                        page.set(Some(default_page));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="posts-content">
            if *loading {
                <div class="loading">{"Loading page..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading page: "}{error_msg}</div>
            } else if let Some(ref page_data) = *page {
                <div class="page-content">
                    {render_page_builder_content_with_navigation_and_context(&page_data.content, Some(props.on_navigate.clone()), page_data.id)}
                </div>
            } else {
                <div class="error">{"Page not found"}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PostContentProps {
    post_id: i32,
    on_admin_click: Callback<()>,
}

#[function_component(PostContent)]
fn post_content(props: &PostContentProps) -> Html {
    let post = use_state(|| None::<crate::services::api_service::Post>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let post = post.clone();
        let loading = loading.clone();
        let error = error.clone();
        let post_id = props.post_id;

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("PostContent: Loading post with ID = {}", post_id).into());
                match crate::services::api_service::get_post(post_id).await {
                    Ok(fetched_post) => {
                        web_sys::console::log_1(&format!("PostContent: Post loaded successfully: {:?}", fetched_post.title).into());
                        post.set(Some(fetched_post));
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("PostContent: Error loading post: {:?}", e).into());
                        let error_message = match e.to_string().as_str() {
                            msg if msg.contains("404") || msg.contains("Not Found") => {
                                format!("Post not found. The post with ID {} may have been deleted or doesn't exist.", post_id)
                            }
                            msg if msg.contains("NetworkError") => {
                                "Unable to connect to the server. Please check your internet connection and try again.".to_string()
                            }
                            _ => format!("Failed to load post: {}", e)
                        };
                        error.set(Some(error_message));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, post_id);
    }

    html! {
        <div class="post-detail">
            if *loading {
                <div class="loading">{"Loading post..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading post: "}{error_msg}</div>
            } else if let Some(ref post_data) = *post {
                <>
                    <h1>{post_data.title.clone()}</h1>
                    <div class="post-meta">
                        <span class="post-author">{"By "}{post_data.author.clone()}</span>
                        if let Some(ref created_at) = post_data.created_at {
                            <span class="post-date">{" • "}{created_at.clone()}</span>
                        }
                        <span class="post-status">{" • "}{post_data.status.clone()}</span>
                    </div>
                    <div class="post-content">
                        <p>{post_data.content.clone()}</p>
                    </div>
                    
                    // Add comments section for posts (if enabled)
                    <ConditionalCommentsSection post_id={Some(post_data.id.unwrap_or(0))} page_id={None} />
                </>
            } else {
                <div class="error">{"Post not found"}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PageContentProps {
    slug: String,
    on_admin_click: Callback<()>,
}

#[function_component(PageContent)]
fn page_content(props: &PageContentProps) -> Html {
    let page = use_state(|| None::<Page>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let page = page.clone();
        let loading = loading.clone();
        let error = error.clone();
        let slug = props.slug.clone();
        let slug_for_deps = slug.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("PageContent: Loading page with slug = {}", slug).into());
                match get_page_by_slug(&slug).await {
                    Ok(fetched_page) => {
                        web_sys::console::log_1(&format!("PageContent: Page loaded successfully: {:?}", fetched_page.title).into());
                        page.set(Some(fetched_page));
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("PageContent: Error loading page: {:?}", e).into());
                        error.set(Some(format!("Failed to load page: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, slug_for_deps);
    }

    html! {
        <div class="page-detail">
            if *loading {
                <div class="loading">{"Loading page..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading page: "}{error_msg}</div>
            } else if let Some(ref page_data) = *page {
                <div class="page-content">
                    {render_page_builder_content_with_navigation_and_context(&page_data.content, None, page_data.id)}
                </div>
            } else {
                <div class="error">{"Page not found"}</div>
            }
        </div>
    }
}

// Function to parse and render page builder content
fn render_page_builder_content(content: &str) -> Html {
    render_page_builder_content_with_navigation(content, None)
}

// Function to parse and render page builder content with navigation callback
fn render_page_builder_content_with_navigation(content: &str, on_navigate: Option<Callback<PublicPage>>) -> Html {
    render_page_builder_content_with_navigation_and_context(content, on_navigate, None)
}

// Function to parse and render page builder content with navigation callback and page context
fn render_page_builder_content_with_navigation_and_context(content: &str, on_navigate: Option<Callback<PublicPage>>, page_id: Option<i32>) -> Html {
    web_sys::console::log_1(&format!("render_page_builder_content_with_navigation called with content length: {}", content.len()).into());
    web_sys::console::log_1(&format!("Content starts with '[': {}", content.starts_with('[')).into());
    
    // TEMPORARY FIX: Check if we're getting malformed content that's just raw JSON as text
    if content.contains("\"component_type\"") && content.contains("\"content\"") && !content.starts_with('[') {
        web_sys::console::log_1(&"Detected malformed JSON content that doesn't start with '[', attempting to fix".into());
        // Try to parse it anyway in case the content is malformed
        match serde_json::from_str::<Vec<PageComponent>>(content) {
            Ok(components) => {
                web_sys::console::log_1(&format!("Successfully parsed {} components from malformed content", components.len()).into());
                html! {
                    <div class="page-builder-content">
                        {components.iter().map(|component| {
                            render_component_content_public_with_context(component, on_navigate.clone(), page_id)
                        }).collect::<Html>()}
                    </div>
                }
            }
            Err(e) => {
                web_sys::console::log_1(&format!("Still failed to parse malformed JSON: {:?}", e).into());
                html! {
                    <div class="error">
                        <h3>{"Page Builder Content Error"}</h3>
                        <p>{"Content appears to be malformed JSON. Please check the page content in the admin panel."}</p>
                        <details>
                            <summary>{"Debug Info"}</summary>
                            <pre>{format!("Error: {:?}\nContent preview: {}", e, &content[0..std::cmp::min(500, content.len())])}</pre>
                        </details>
                    </div>
                }
            }
        }
    } else if content.starts_with('[') {
        match serde_json::from_str::<Vec<PageComponent>>(content) {
            Ok(components) => {
                web_sys::console::log_1(&format!("Successfully parsed {} components", components.len()).into());
                html! {
                    <div class="page-builder-content">
                        {components.iter().map(|component| {
                            render_component_content_public_with_context(component, on_navigate.clone(), page_id)
                        }).collect::<Html>()}
                    </div>
                }
            }
            Err(e) => {
                web_sys::console::log_1(&format!("Failed to parse JSON: {:?}", e).into());
                web_sys::console::log_1(&format!("Content preview: {}", &content[0..std::cmp::min(200, content.len())]).into());
                // Show error message instead of fallback to avoid confusion
                html! {
                    <div class="error">
                        <h3>{"Page Builder Content Error"}</h3>
                        <p>{"Failed to parse page components. Please check the page content in the admin panel."}</p>
                        <details>
                            <summary>{"Debug Info"}</summary>
                            <pre>{format!("Error: {:?}\nContent preview: {}", e, &content[0..std::cmp::min(200, content.len())])}</pre>
                        </details>
                    </div>
                }
            }
        }
    } else {
        web_sys::console::log_1(&"Content does not start with '[', rendering as markdown".into());
        // Regular content, render as markdown
        render_markdown_content(content)
    }
}



// Comprehensive component renderer for public pages with navigation callback
pub fn render_component_content_public_with_navigation(component: &PageComponent, on_navigate: Option<Callback<PublicPage>>) -> Html {
    render_component_content_public_with_context(component, on_navigate, None)
}

// Enhanced component renderer with page context for Comments component
pub fn render_component_content_public_with_context(component: &PageComponent, on_navigate: Option<Callback<PublicPage>>, page_id: Option<i32>) -> Html {
    match component.component_type {
        ComponentType::Sidebar => {
            // Render a left/right two-column layout wrapper for public view
            let is_left = component.properties.sidebar_position == "left";
            let grid_style = if is_left {
                format!("display: grid; grid-template-columns: {} 1fr; gap: 16px; {}", component.properties.sidebar_width, format_component_styles(&component.styles))
            } else {
                format!("display: grid; grid-template-columns: 1fr {}; gap: 16px; {}", component.properties.sidebar_width, format_component_styles(&component.styles))
            };
            let rail = html! {
                <aside class="sidebar-rail" style="border: 1px solid #eee; border-radius: 8px; padding: 12px; background: #fafafa;">
                    <div style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Sidebar"}</div>
                </aside>
            };
            let body_children = if is_left { &component.properties.column_2_components } else { &component.properties.column_1_components };
            html! {
                <div class="component sidebar-layout" style={grid_style}>
                    { if is_left { rail.clone() } else { html!{} } }
                    <div class="page-body">
                        {body_children.iter().map(|nested_comp| {
                            render_component_content_public_with_context(nested_comp, on_navigate.clone(), page_id)
                        }).collect::<Html>()}
                    </div>
                    { if !is_left { rail } else { html!{} } }
                </div>
            }
        }
        ComponentType::Text => {
            html! {
                <div class="component text-component" style={format_component_styles(&component.styles)}>
                    <p>{&component.content}</p>
                </div>
            }
        }
        ComponentType::Heading => {
            html! {
                <div class="component heading-component" style={format_component_styles(&component.styles)}>
                    <h1>{&component.content}</h1>
                </div>
            }
        }
        ComponentType::Subheading => {
            html! {
                <div class="component subheading-component" style={format_component_styles(&component.styles)}>
                    <h2>{&component.content}</h2>
                </div>
            }
        }
        ComponentType::Hero => {
            // Skip rendering Hero components that have default/empty properties
            // This handles the case where a Hero component exists but shouldn't be displayed
            let has_hero_properties = !component.properties.hero_title.is_empty() || 
                                    !component.properties.hero_description.is_empty() || 
                                    !component.properties.hero_subtitle.is_empty();
            
            let has_meaningful_content = !component.content.trim().is_empty() && 
                                       component.content.trim() != "# Welcome to Rust CMS\n\nExperience the power of WebAssembly and Rust in a clean, minimalist CMS designed for modern web development.";
            
            // Only render if it has meaningful hero properties OR non-default content
            if !has_hero_properties && !has_meaningful_content {
                web_sys::console::log_1(&format!("Live Edit: Skipping Hero component '{}' - no meaningful content or properties", component.id).into());
                return html! { <></> };
            }
            
            web_sys::console::log_1(&format!("Live Edit: Rendering Hero component '{}' - has_props: {}, has_content: {}", 
                component.id, has_hero_properties, has_meaningful_content).into());
            
            // Dynamic hero using component properties
            let background_style = match component.properties.hero_background_type.as_str() {
                "gradient" => format!(
                    "background: linear-gradient(135deg, {} 0%, {} 100%);",
                    component.properties.hero_background_gradient_start,
                    component.properties.hero_background_gradient_end
                ),
                "solid" => format!("background: {};", component.properties.hero_background_color),
                "image" => if !component.properties.hero_background_image.is_empty() {
                    format!(
                        "background: linear-gradient(135deg, {}66 0%, {}66 100%), url({}) center/cover;",
                        component.properties.hero_background_color,
                        component.properties.hero_background_color,
                        component.properties.hero_background_image
                    )
                } else {
                    format!("background: {};", component.properties.hero_background_color)
                },
                _ => format!(
                    "background: linear-gradient(135deg, {} 0%, {} 100%);",
                    component.properties.hero_background_gradient_start,
                    component.properties.hero_background_gradient_end
                ),
            };
            
            let hero_style = format!(
                "{} {}; color: {}; padding: {}; text-align: {}; border-radius: 12px; position: relative; overflow: hidden; min-height: {}; z-index: 1;",
                background_style,
                format_component_styles(&component.styles),
                component.properties.hero_text_color,
                component.properties.hero_padding,
                component.properties.hero_alignment,
                component.properties.hero_min_height
            );
            
            html! {
                <section class="component hero-section" style={hero_style}>
                    // Background pattern
                    <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; opacity: 0.1; background-image: radial-gradient(circle at 25% 25%, white 2px, transparent 2px), radial-gradient(circle at 75% 75%, white 2px, transparent 2px); background-size: 50px 50px;"></div>
                    
                    <div class="hero-content" style="position: relative; z-index: 1; max-width: 800px; margin: 0 auto;">
                        // Hero badge (conditional)
                        if component.properties.hero_show_badge && !component.properties.hero_badge_text.is_empty() {
                            <div class="hero-badge" style="display: inline-block; background: rgba(255,255,255,0.2); padding: 8px 16px; border-radius: 20px; font-size: 14px; margin-bottom: 24px; border: 1px solid rgba(255,255,255,0.3);">
                                {&component.properties.hero_badge_text}
                            </div>
                        }
                        
                        // Hero title
                        <h1 style="font-size: 48px; font-weight: 700; margin: 0 0 24px 0; line-height: 1.2;">
                            {&component.properties.hero_title}
                        </h1>
                        
                        // Hero subtitle (conditional)
                        if !component.properties.hero_subtitle.is_empty() {
                            <h2 style="font-size: 28px; font-weight: 400; margin: 0 0 24px 0; opacity: 0.9;">
                                {&component.properties.hero_subtitle}
                            </h2>
                        }
                        
                        // Hero description
                        <p style="font-size: 20px; margin: 0 0 32px 0; opacity: 0.9; line-height: 1.6;">
                            {&component.properties.hero_description}
                        </p>
                        
                        // Hero action buttons (conditional)
                        if component.properties.hero_show_primary_button || component.properties.hero_show_secondary_button {
                            <div class="hero-actions" style="display: flex; gap: 16px; justify-content: center; flex-wrap: wrap;">
                                if component.properties.hero_show_primary_button && !component.properties.hero_primary_button_text.is_empty() {
                                    <a 
                                        href={component.properties.hero_primary_button_url.clone()}
                                        style="display: inline-flex; align-items: center; gap: 8px; padding: 16px 32px; background: white; color: var(--public-primary, #3b82f6); text-decoration: none; border-radius: 8px; font-weight: 600; font-size: 16px; transition: all 0.2s; box-shadow: 0 4px 12px rgba(0,0,0,0.1);"
                                    >
                                        {&component.properties.hero_primary_button_text} <span>{"→"}</span>
                                    </a>
                                }
                                if component.properties.hero_show_secondary_button && !component.properties.hero_secondary_button_text.is_empty() {
                                    <a 
                                        href={component.properties.hero_secondary_button_url.clone()}
                                        style="display: inline-flex; align-items: center; gap: 8px; padding: 16px 32px; background: transparent; color: white; text-decoration: none; border: 2px solid rgba(255,255,255,0.3); border-radius: 8px; font-weight: 600; font-size: 16px; transition: all 0.2s;"
                                    >
                                        {&component.properties.hero_secondary_button_text}
                                    </a>
                                }
                            </div>
                        }
                        
                        // Hero stats (conditional)
                        if component.properties.hero_show_stats && (!component.properties.hero_stat1_number.is_empty() || !component.properties.hero_stat2_number.is_empty() || !component.properties.hero_stat3_number.is_empty()) {
                            <div class="hero-stats" style="margin-top: 48px; display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 24px; opacity: 0.9;">
                                if !component.properties.hero_stat1_number.is_empty() {
                                    <div class="stat-item" style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{&component.properties.hero_stat1_number}</div>
                                        <div style="font-size: 14px; opacity: 0.8;">{&component.properties.hero_stat1_label}</div>
                                    </div>
                                }
                                if !component.properties.hero_stat2_number.is_empty() {
                                    <div class="stat-item" style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{&component.properties.hero_stat2_number}</div>
                                        <div style="font-size: 14px; opacity: 0.8;">{&component.properties.hero_stat2_label}</div>
                                    </div>
                                }
                                if !component.properties.hero_stat3_number.is_empty() {
                                    <div class="stat-item" style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{&component.properties.hero_stat3_number}</div>
                                        <div style="font-size: 14px; opacity: 0.8;">{&component.properties.hero_stat3_label}</div>
                                    </div>
                                }
                            </div>
                        }
                    </div>
                </section>
            }
        }
        ComponentType::Card => {
            // Use component properties for dynamic content and post-card styling
            let card_style = format!(
                "background: {}; border-radius: {}; padding: {}; box-shadow: {}; border: 1px solid #eee; transition: transform 0.2s ease, box-shadow 0.2s ease; {}",
                component.properties.card_background,
                component.properties.card_border_radius,
                component.properties.card_padding,
                match component.properties.card_shadow.as_str() {
                    "none" => "none",
                    "small" => "0 2px 4px rgba(0, 0, 0, 0.05)",
                    "medium" => "0 4px 12px rgba(0, 0, 0, 0.1)",
                    "large" => "0 8px 32px rgba(0, 0, 0, 0.15)",
                    _ => "0 4px 12px rgba(0, 0, 0, 0.1)",
                },
                format_component_styles(&component.styles)
            );
            
            html! {
                <div class="component post-card" style={card_style}>
                    // Card image if provided
                    if !component.properties.card_image.is_empty() {
                        <div class="card-image" style="margin-bottom: 1rem;">
                            <img 
                                src={component.properties.card_image.clone()} 
                                alt={component.properties.card_image_alt.clone()}
                                style="width: 100%; height: 200px; object-fit: cover; border-radius: 4px;"
                            />
                        </div>
                    }
                    
                    // Card title
                    <h4 style="font-size: 1.3rem; margin-bottom: 0.5rem; color: var(--public-heading-h3, #000);">
                        {&component.properties.card_title}
                    </h4>
                    
                    // Card meta text if provided
                    if !component.properties.card_meta_text.is_empty() {
                        <div class="post-meta" style="color: var(--public-text-meta, #666); font-size: 0.9rem; margin-bottom: 1rem;">
                            {&component.properties.card_meta_text}
                        </div>
                    }
                    
                    // Card description
                    <div class="post-excerpt" style="color: var(--public-text-secondary, #555); margin-bottom: 1rem;">
                        {&component.properties.card_description}
                    </div>
                    
                    // Card button if enabled
                    if component.properties.card_button_show && !component.properties.card_button_text.is_empty() {
                        <a 
                            href={component.properties.card_button_url.clone()}
                            class="read-more"
                            style="color: var(--public-link-primary, #000); text-decoration: none; font-weight: 600; border-bottom: 2px solid var(--public-link-primary, #000); transition: border-color 0.2s ease;"
                        >
                            {&component.properties.card_button_text}
                        </a>
                    }
                </div>
            }
        }
        ComponentType::Button => {
            let button_url = &component.properties.button_url;
            let button_target = &component.properties.button_target;
            let button_text = if component.properties.button_text.is_empty() {
                &component.content
            } else {
                &component.properties.button_text
            };
            
            // Enhanced button styling for public pages - ensure visibility
            let button_style = format!(
                "{}; display: inline-block; padding: 12px 24px; background-color: #007bff; color: white; text-decoration: none; cursor: pointer; border: 1px solid #007bff; border-radius: 4px; font-weight: 500; text-align: center; transition: all 0.2s ease; min-width: 80px;",
                format_component_styles(&component.styles)
            );
            
            let href = if button_url.is_empty() { "#".to_string() } else { button_url.clone() };
            let target = if button_target.is_empty() { "_self".to_string() } else { button_target.clone() };
            
            html! {
                <div class="component button-component" style="margin: 8px 0;">
                    <a href={href} target={target} class="btn" style={button_style}>
                        {button_text}
                    </a>
                </div>
            }
        }
        ComponentType::Link => {
            let button_url = &component.properties.button_url;
            let button_target = &component.properties.button_target;
            let link_text = if component.properties.button_text.is_empty() {
                &component.content
            } else {
                &component.properties.button_text
            };
            
            html! {
                <div class="component link-component" style={format_component_styles(&component.styles)}>
                    <a href={button_url.clone()} target={button_target.clone()}>
                        {link_text}
                    </a>
                </div>
            }
        }
        ComponentType::Container => {
            let max_width = &component.properties.container_max_width;
            let align = &component.properties.container_align;
            
            let container_style = format!(
                "{}; max-width: {}; margin: 0 {}; width: 100%;",
                format_component_styles(&component.styles),
                max_width,
                if align == "center" { "auto" } else if align == "right" { "0 0 0 auto" } else { "0 auto 0 0" }
            );
            
            let effects_class = if component.properties.effects != "none" {
                format!("effect-{}", component.properties.effects)
            } else {
                String::new()
            };
            
            let effects_style = if component.properties.effects != "none" {
                format!("--multiply-intensity: {};", component.properties.effects_intensity)
            } else {
                String::new()
            };
            
            html! {
                <div class={format!("component container-component container {}", effects_class)} 
                     style={format!("{}; {}", container_style, effects_style)} 
                     data-opacity={if component.properties.effects != "none" { "true" } else { "false" }}>
                    {if !component.properties.nested_components.is_empty() {
                        html! {
                            <div class="nested-components">
                                {component.properties.nested_components.iter().map(|nested_comp| {
                                    render_component_content_public_with_context(nested_comp, on_navigate.clone(), page_id)
                                }).collect::<Html>()}
                            </div>
                        }
                    } else {
                        // Fallback to content-based rendering for backward compatibility
                        render_nested_content(&component.content, on_navigate.clone(), page_id)
                    }}
                </div>
            }
        }
        ComponentType::TwoColumn => {
            let column_style = format!(
                "display: grid; grid-template-columns: 1fr 1fr; gap: 20px; {}",
                format_component_styles(&component.styles)
            );
            
            // Use new nested components structure if available
            if !component.properties.column_1_components.is_empty() || !component.properties.column_2_components.is_empty() {
                html! {
                    <div class="component two-column-component" style={column_style}>
                        <div class="column">
                            {component.properties.column_1_components.iter().map(|nested_comp| {
                                render_component_content_public_with_context(nested_comp, on_navigate.clone(), page_id)
                            }).collect::<Html>()}
                        </div>
                        <div class="column">
                            {component.properties.column_2_components.iter().map(|nested_comp| {
                                render_component_content_public_with_context(nested_comp, on_navigate.clone(), page_id)
                            }).collect::<Html>()}
                        </div>
                    </div>
                }
            } else {
                // Fallback to content-based rendering for backward compatibility
                
                // Try to parse as nested column content first
                if component.content.trim().starts_with('{') && component.content.contains("\"column1\"") && component.content.contains("\"column2\"") {
                    match serde_json::from_str::<serde_json::Value>(&component.content) {
                        Ok(columns_data) => {
                            let column1_content = columns_data.get("column1").and_then(|v| v.as_str()).unwrap_or("");
                            let column2_content = columns_data.get("column2").and_then(|v| v.as_str()).unwrap_or("");
                            
                            return html! {
                                <div class="component two-column-component" style={column_style}>
                                    <div class="column">
                                        {render_nested_content(column1_content, on_navigate.clone(), page_id)}
                                    </div>
                                    <div class="column">
                                        {render_nested_content(column2_content, on_navigate.clone(), page_id)}
                                    </div>
                                </div>
                            }
                        }
                        Err(_) => {
                            // Fall back to text splitting
                        }
                    }
                }
                
                // Fall back to original text splitting approach
                let parts: Vec<&str> = component.content.split("\n\n").collect();
                html! {
                    <div class="component two-column-component" style={column_style}>
                        <div class="column">
                            {render_markdown_content(parts.get(0).unwrap_or(&""))}
                        </div>
                        <div class="column">
                            {render_markdown_content(parts.get(1).unwrap_or(&""))}
                        </div>
                    </div>
                }
            }
        }
        ComponentType::ThreeColumn => {
            let column_style = format!(
                "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px; {}",
                format_component_styles(&component.styles)
            );
            
            // Use new nested components structure if available
            if !component.properties.column_1_components.is_empty() || !component.properties.column_2_components.is_empty() || !component.properties.column_3_components.is_empty() {
                html! {
                    <div class="component three-column-component" style={column_style}>
                        <div class="column">
                            {component.properties.column_1_components.iter().map(|nested_comp| {
                                render_component_content_public_with_context(nested_comp, on_navigate.clone(), page_id)
                            }).collect::<Html>()}
                        </div>
                        <div class="column">
                            {component.properties.column_2_components.iter().map(|nested_comp| {
                                render_component_content_public_with_context(nested_comp, on_navigate.clone(), page_id)
                            }).collect::<Html>()}
                        </div>
                        <div class="column">
                            {component.properties.column_3_components.iter().map(|nested_comp| {
                                render_component_content_public_with_context(nested_comp, on_navigate.clone(), page_id)
                            }).collect::<Html>()}
                        </div>
                    </div>
                }
            } else {
                // Fallback to content-based rendering for backward compatibility
                
                // Try to parse as nested column content first
                if component.content.trim().starts_with('{') && component.content.contains("\"column1\"") && component.content.contains("\"column2\"") && component.content.contains("\"column3\"") {
                    match serde_json::from_str::<serde_json::Value>(&component.content) {
                        Ok(columns_data) => {
                            let column1_content = columns_data.get("column1").and_then(|v| v.as_str()).unwrap_or("");
                            let column2_content = columns_data.get("column2").and_then(|v| v.as_str()).unwrap_or("");
                            let column3_content = columns_data.get("column3").and_then(|v| v.as_str()).unwrap_or("");
                            
                            return html! {
                                <div class="component three-column-component" style={column_style}>
                                    <div class="column">
                                        {render_nested_content(column1_content, on_navigate.clone(), page_id)}
                                    </div>
                                    <div class="column">
                                        {render_nested_content(column2_content, on_navigate.clone(), page_id)}
                                    </div>
                                    <div class="column">
                                        {render_nested_content(column3_content, on_navigate.clone(), page_id)}
                                    </div>
                                </div>
                            }
                        }
                        Err(_) => {
                            // Fall back to text splitting
                        }
                    }
                }
                
                // Fall back to original text splitting approach
                let parts: Vec<&str> = component.content.split("\n\n").collect();
                html! {
                    <div class="component three-column-component" style={column_style}>
                        <div class="column">
                            {render_markdown_content(parts.get(0).unwrap_or(&""))}
                        </div>
                        <div class="column">
                            {render_markdown_content(parts.get(1).unwrap_or(&""))}
                        </div>
                        <div class="column">
                            {render_markdown_content(parts.get(2).unwrap_or(&""))}
                        </div>
                    </div>
                }
            }
        }
        ComponentType::Image => {
            let image_url = &component.properties.image_url;
            let image_alt = &component.properties.image_alt;
            let image_title = &component.properties.image_title;
            
            // Check if this is an SVG file
            let is_svg = image_url.to_lowercase().ends_with(".svg") || 
                        image_url.to_lowercase().contains("image/svg+xml");
            
            html! {
                <div class="component image-component" style={format_component_styles(&component.styles)}>
                    <img 
                        src={image_url.clone()} 
                        alt={image_alt.clone()} 
                        title={image_title.clone()}
                        style={format!(
                            "max-width: 100%; height: auto; display: block;{}",
                            if is_svg { " vector-effect: non-scaling-stroke;" } else { "" }
                        )}
                        // Add loading attribute for better performance
                        loading="lazy"
                        // Add decoding attribute for better rendering
                        decoding="async"
                    />
                    if !component.content.is_empty() {
                        <figcaption style="margin-top: 8px; font-style: italic; color: #666;">
                            {&component.content}
                        </figcaption>
                    }
                </div>
            }
        }
        ComponentType::List => {
            let list_style = format!(
                "background: {}; border-radius: {}; padding: {}; border: 1px solid #e1e5e9; {}",
                component.properties.list_background,
                component.properties.list_border_radius,
                component.properties.list_padding,
                format_component_styles(&component.styles)
            );
            
            html! {
                <div class="component enhanced-list" style={list_style}>
                    <div class="list-items" style={format!("display: grid; gap: {};", component.properties.list_item_spacing)}>
                        {component.properties.list_items.iter().enumerate().map(|(index, item)| {
                            // Icon colors for visual variety
                            let icon_colors = [
                                "linear-gradient(135deg, #10b981, #059669)", // Green
                                "linear-gradient(135deg, #3b82f6, #1d4ed8)", // Blue  
                                "linear-gradient(135deg, #8b5cf6, #7c3aed)", // Purple
                                "linear-gradient(135deg, #f59e0b, #d97706)", // Orange
                                "linear-gradient(135deg, #ef4444, #dc2626)", // Red
                                "linear-gradient(135deg, #06b6d4, #0891b2)", // Cyan
                            ];
                            let icon_gradient = icon_colors[index % icon_colors.len()];
                            
                            html! {
                                <div class="list-item" style="display: flex; align-items: flex-start; gap: 16px; padding: 16px; background: #f8f9fa; border-radius: 8px; transition: all 0.2s;">
                                    if component.properties.list_show_icons {
                                        <div class="item-icon" style={format!("flex-shrink: 0; width: 40px; height: 40px; background: {}; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; font-size: 18px;", icon_gradient)}>
                                            {&item.icon}
                                        </div>
                                    }
                                    <div class="item-content" style="flex: 1;">
                                        <h4 style={format!("margin: 0 0 4px 0; font-size: 16px; font-weight: 600; color: #333333;")}>
                                            {&item.title}
                                        </h4>
                                        if !item.description.is_empty() {
                                            <p style={format!("margin: 0; color: {}; font-size: 14px; line-height: 1.5;", component.properties.list_text_color)}>
                                                {&item.description}
                                            </p>
                                        }
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        }
        ComponentType::Quote => {
            html! {
                <div class="component quote-component" style={format_component_styles(&component.styles)}>
                    <blockquote style="margin: 0; padding-left: 20px; border-left: 4px solid #ddd; font-style: italic;">
                        {render_markdown_content(&component.content)}
                    </blockquote>
                </div>
            }
        }
        ComponentType::Spacer => {
            let height = if component.content.is_empty() { "20px" } else { &component.content };
            html! {
                <div class="component spacer-component" style={format!("height: {}; {}", height, format_component_styles(&component.styles))}>
                </div>
            }
        }
        ComponentType::Divider => {
            let thickness = &component.properties.divider_thickness;
            let color = &component.properties.divider_color;
            let divider_style = &component.properties.divider_style;
            let width = &component.properties.divider_width;
            
            let hr_style = format!(
                "border: none; border-top: {} {} {}; width: {}; margin: 0; {}",
                thickness, divider_style, color, width, format_component_styles(&component.styles)
            );
            
            html! {
                <div class="component divider-component">
                    <hr style={hr_style} />
                </div>
            }
        }
        ComponentType::Video => {
            let video_url = &component.properties.video_url;
            
            if video_url.is_empty() {
                // Show placeholder for video component with no URL
                html! {
                    <div class="component video-component" style={format_component_styles(&component.styles)}>
                        <div class="placeholder-video" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 60px 40px; text-align: center; border-radius: 8px; color: var(--public-text-secondary, #666);">
                            <div style="font-size: 48px; margin-bottom: 16px;">{"🎥"}</div>
                            <div style="font-size: 18px; font-weight: 500; margin-bottom: 8px;">{"Video Component"}</div>
                            <div style="font-size: 14px;">{"Configure video URL in properties to display video"}</div>
                        </div>
                    </div>
                }
            } else {
                let autoplay = component.properties.video_autoplay;
                let controls = component.properties.video_controls;
                let muted = component.properties.video_muted;
                let loop_video = component.properties.video_loop;
                
                html! {
                    <div class="component video-component" style={format_component_styles(&component.styles)}>
                        <video 
                            src={video_url.clone()}
                            controls={controls}
                            autoplay={autoplay}
                            muted={muted}
                            loop={loop_video}
                            style="width: 100%; height: auto;"
                        >
                            {"Your browser does not support the video tag."}
                        </video>
                        // Note: Removed text content display for video components - videos should not show text
                    </div>
                }
            }
        }
        ComponentType::PostsList => {
            // Parse properties for PostsList configuration
            let posts_to_show = component.properties.container_max_width.parse::<usize>().unwrap_or(6);
            let show_full = component.properties.container_max_width == "all" || posts_to_show >= 100;
            let excerpt_length = component.properties.divider_margin.parse::<usize>().unwrap_or(200);
            let _grid_columns = component.properties.gallery_columns;
            
            // Create custom styling based on component properties
            let card_bg = &component.properties.button_text;
            let card_radius = &component.properties.button_size;
            let grid_gap = &component.properties.button_url;
            let title_color = &component.properties.button_icon;
            let meta_color = &component.properties.form_action;
            let link_color = &component.properties.form_method;
            let shadow_type = &component.properties.button_variant;
            
            // Generate shadow CSS based on selection
            let card_shadow = match shadow_type.as_str() {
                "none" => "none",
                "small" => "0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24)",
                "large" => "0 14px 28px rgba(0, 0, 0, 0.25), 0 10px 10px rgba(0, 0, 0, 0.22)",
                "design-system" => "var(--public-card-shadow, 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05))",
                _ => "0 4px 6px rgba(0, 0, 0, 0.07), 0 2px 4px rgba(0, 0, 0, 0.05)", // medium/default
            };
            
            // Create dynamic styling for the posts grid
            let posts_grid_style = format!(
                "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: {}; max-width: 1200px; margin: 0 auto;",
                grid_gap
            );
            
            let post_card_style = format!(
                "background: {}; border-radius: {}; box-shadow: {}; transition: transform 0.2s ease, box-shadow 0.2s ease; overflow: hidden;",
                card_bg, card_radius, card_shadow
            );
            
            html! {
                <div class="component posts-list-component" style={format_component_styles(&component.styles)}>
                    <style>
                        {format!(r#"
                            .posts-list-widget .posts-grid {{
                                {}
                            }}
                            .posts-list-widget .post-card {{
                                {}
                            }}
                            .posts-list-widget .post-card h2 {{
                                color: {} !important;
                            }}
                            .posts-list-widget .post-meta {{
                                color: {} !important;
                            }}
                            .posts-list-widget .read-more {{
                                color: {} !important;
                            }}
                            .posts-list-widget .post-card:hover {{
                                transform: translateY(-2px);
                                box-shadow: 0 8px 25px rgba(0,0,0,0.15);
                            }}
                        "#, posts_grid_style, post_card_style, title_color, meta_color, link_color)}
                    </style>
                    <PostsListWidget 
                        show_full_list={show_full} 
                        limit={posts_to_show} 
                        excerpt_length={excerpt_length}
                        on_navigate={on_navigate.clone()}
                    />
                </div>
            }
        }
        ComponentType::Gallery => {
            html! {
                <div class="component gallery-component" style={format_component_styles(&component.styles)}>
                    <EnhancedGallery 
                        images={component.properties.gallery_images.iter().map(|img| EnhancedGalleryImage {
                            id: uuid::Uuid::new_v4().to_string(),
                            url: img.url.clone(),
                            alt: img.alt.clone(),
                            caption: img.caption.clone(),
                            title: img.title.clone(),
                            media_id: None,
                        }).collect::<Vec<_>>()}
                        on_images_change={Callback::noop()} // Read-only for public pages
                        layout={component.properties.gallery_layout.clone()}
                        columns={component.properties.gallery_columns}
                        gap={component.properties.gallery_gap}
                        border_radius={component.properties.gallery_border_radius}
                        show_captions={component.properties.gallery_show_captions}
                        enable_lightbox={component.properties.gallery_enable_lightbox}
                        enable_drag_reorder={false} // Disable drag reorder on public pages
                        is_admin_mode={false} // Public view behavior
                    />
                </div>
            }
        }
        ComponentType::ContactForm => {
            html! {
                <div class="component contact-form-component" style={format_component_styles(&component.styles)}>
                    <form>
                        <div style="margin-bottom: 16px;">
                            <label for="name" style="display: block; margin-bottom: 4px;">{"Name"}</label>
                            <input type="text" id="name" name="name" style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;" />
                        </div>
                        <div style="margin-bottom: 16px;">
                            <label for="email" style="display: block; margin-bottom: 4px;">{"Email"}</label>
                            <input type="email" id="email" name="email" style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;" />
                        </div>
                        <div style="margin-bottom: 16px;">
                            <label for="message" style="display: block; margin-bottom: 4px;">{"Message"}</label>
                            <textarea id="message" name="message" rows="5" style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; resize: vertical;"></textarea>
                        </div>
                        <button type="submit" style="background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer;">
                            {"Send Message"}
                        </button>
                    </form>
                </div>
            }
        }
        ComponentType::Newsletter => {
            html! {
                <div class="component newsletter-component" style={format_component_styles(&component.styles)}>
                    <div style="text-align: center;">
                        if !component.content.is_empty() {
                            <p>{&component.content}</p>
                        } else {
                            <p>{"Subscribe to our newsletter for updates!"}</p>
                        }
                        <form style="display: flex; gap: 8px; justify-content: center; max-width: 400px; margin: 0 auto;">
                            <input 
                                type="email" 
                                placeholder="Enter your email" 
                                style="flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px;"
                            />
                            <button 
                                type="submit" 
                                style="background: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; white-space: nowrap;"
                            >
                                {"Subscribe"}
                            </button>
                        </form>
                    </div>
                </div>
            }
        }
        ComponentType::Map => {
            html! {
                <div class="component map-component" style={format_component_styles(&component.styles)}>
                    <div style="background: #f0f0f0; padding: 40px; text-align: center; border-radius: 8px; color: #666;">
                        <p>{"🗺️ Map Component"}</p>
                        <p>{"Map integration would be implemented here"}</p>
                        if !component.content.is_empty() {
                            <p>{&component.content}</p>
                        }
                    </div>
                </div>
            }
        }
        ComponentType::Comments => {
            // Render comments component with proper context (page vs post)
            let show_auth_prompt = component.properties.comments_show_auth_prompt;
            
            // If we have a page_id context, use it for page comments
            // Otherwise, fall back to post_id from component properties
            let (final_post_id, final_page_id) = if let Some(current_page_id) = page_id {
                (None, Some(current_page_id))
            } else {
                (Some(component.properties.comments_post_id), None)
            };
            
            html! {
                <div class="component comments-component" style={format_component_styles(&component.styles)}>
                    <CommentsSection 
                        post_id={final_post_id}
                        page_id={final_page_id}
                        show_auth_prompt={show_auth_prompt}
                    />
                </div>
            }
        }
        ComponentType::PageTitle => {
            let tag = &component.properties.page_title_tag;
            let prefix = if component.properties.page_title_show_prefix && !component.properties.page_title_prefix.is_empty() {
                format!("{} ", component.properties.page_title_prefix)
            } else {
                String::new()
            };
            let suffix = if component.properties.page_title_show_suffix && !component.properties.page_title_suffix.is_empty() {
                format!(" {}", component.properties.page_title_suffix)
            } else {
                String::new()
            };
            
            // Use placeholder text for now - users can edit this in the page builder
            let page_title = "Page Title".to_string();
            
            let title_content = format!("{}{}{}", prefix, page_title, suffix);
            
            match tag.as_str() {
                "h1" => html! { <h1 class="page-title" style={format_component_styles(&component.styles)}>{title_content}</h1> },
                "h2" => html! { <h2 class="page-title" style={format_component_styles(&component.styles)}>{title_content}</h2> },
                "h3" => html! { <h3 class="page-title" style={format_component_styles(&component.styles)}>{title_content}</h3> },
                "h4" => html! { <h4 class="page-title" style={format_component_styles(&component.styles)}>{title_content}</h4> },
                "h5" => html! { <h5 class="page-title" style={format_component_styles(&component.styles)}>{title_content}</h5> },
                "h6" => html! { <h6 class="page-title" style={format_component_styles(&component.styles)}>{title_content}</h6> },
                _ => html! { <h1 class="page-title" style={format_component_styles(&component.styles)}>{title_content}</h1> },
            }
        }
        ComponentType::PublishedDate => {
            let prefix = if component.properties.published_date_show_prefix && !component.properties.published_date_prefix.is_empty() {
                format!("{} ", component.properties.published_date_prefix)
            } else {
                String::new()
            };
            
            // In a real implementation, this would get the actual page published date
            let date_content = if component.properties.published_date_relative {
                format!("{}2 days ago", prefix)
            } else {
                match component.properties.published_date_format.as_str() {
                    "YYYY-MM-DD" => format!("{}2024-08-25", prefix),
                    "DD/MM/YYYY" => format!("{}25/08/2024", prefix),
                    "MM/DD/YYYY" => format!("{}08/25/2024", prefix),
                    "Month DD, YYYY" => format!("{}August 25, 2024", prefix),
                    _ => format!("{}August 25, 2024", prefix),
                }
            };
            
            let time_suffix = if component.properties.published_date_show_time {
                " at 2:30 PM"
            } else {
                ""
            };
            
            html! {
                <div class="published-date" style={format!("color: var(--public-text-secondary, #666); font-size: 0.9em; {}", format_component_styles(&component.styles))}>
                    {format!("{}{}", date_content, time_suffix)}
                </div>
            }
        }
    }
}

// Helper function to render nested components or markdown content
fn render_nested_content(content: &str, on_navigate: Option<Callback<PublicPage>>, page_id: Option<i32>) -> Html {
    // Try to parse as JSON array of nested components first
    if content.trim().starts_with('[') && content.trim().ends_with(']') {
        match serde_json::from_str::<Vec<PageComponent>>(content) {
            Ok(nested_components) => {
                return html! {
                    <div class="nested-components">
                        {nested_components.iter().map(|component| {
                            render_component_content_public_with_context(component, on_navigate.clone(), page_id)
                        }).collect::<Html>()}
                    </div>
                }
            }
            Err(_) => {
                // Fall back to markdown if JSON parsing fails
            }
        }
    }
    
    // Render as markdown
    render_markdown_content(content)
}

// Helper function to render markdown content
pub fn render_markdown_content(content: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Html::from_html_unchecked(html_output.into())
}

// Helper function to format component styles
pub fn format_component_styles(styles: &crate::components::page_builder::ComponentStyles) -> String {
    format!(
        "background-color: {}; color: {}; padding: {}; margin: {}; border-radius: {}; font-size: {}; font-weight: {}; text-align: {}; border: {}px {} {}; opacity: {}; z-index: {}; font-family: {}; line-height: {}; letter-spacing: {}; text-decoration: {}; text-transform: {};",
        styles.background_color,
        styles.text_color,
        styles.padding,
        styles.margin,
        styles.border_radius,
        styles.font_size,
        styles.font_weight,
        styles.text_align,
        styles.border_width,
        styles.border_style,
        styles.border_color,
        styles.opacity,
        styles.z_index,
        styles.font_family,
        styles.line_height,
        styles.letter_spacing,
        styles.text_decoration,
        styles.text_transform
    )
}

#[derive(Properties, PartialEq)]
struct ConditionalCommentsSectionProps {
    pub post_id: Option<i32>,
    pub page_id: Option<i32>,
}

#[function_component(ConditionalCommentsSection)]
fn conditional_comments_section(props: &ConditionalCommentsSectionProps) -> Html {
    let comments_enabled = use_state(|| true);
    let loading = use_state(|| true);

    {
        let comments_enabled = comments_enabled.clone();
        let loading = loading.clone();
        
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let enabled = check_comments_enabled().await;
                comments_enabled.set(enabled);
                loading.set(false);
            });
            || ()
        }, ());
    }

    if *loading {
        return html! { <div></div> }; // Don't show anything while loading
    }

    if *comments_enabled {
        html! {
            <CommentsSection 
                post_id={props.post_id}
                page_id={props.page_id}
                show_auth_prompt={true}
            />
        }
    } else {
        html! { <div></div> } // Comments are disabled via template
    }
} 