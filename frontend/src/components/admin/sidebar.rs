use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum AdminTab {
    Dashboard,
    Posts,
    PostCreate,
    Pages,
    Media,
    Users,
    Comments,
    Navigation,
    Templates,
    Analytics,
    Plugins,
    SystemSettings,
    DesignSystem,
}

#[derive(Properties, PartialEq)]
pub struct AdminSidebarProps {
    pub on_tab_click: Callback<AdminTab>,
    pub active_tab: AdminTab,
    pub on_public_click: Callback<()>,
}

#[function_component(AdminSidebar)]
pub fn admin_sidebar(props: &AdminSidebarProps) -> Html {
    let on_dashboard_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Dashboard))
    };

    let on_posts_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Posts))
    };

    let on_pages_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Pages))
    };

    let on_media_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Media))
    };

    let on_users_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Users))
    };

    let on_comments_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Comments))
    };

    let on_navigation_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Navigation))
    };

    let on_templates_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Templates))
    };

    let on_analytics_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Analytics))
    };

    let on_plugins_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::Plugins))
    };

    let on_system_settings_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::SystemSettings))
    };

    let on_design_system_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(AdminTab::DesignSystem))
    };

    let on_public_click = {
        let on_public_click = props.on_public_click.clone();
        Callback::from(move |_| on_public_click.emit(()))
    };

    html! {
        <nav class="admin-sidebar">
            <div class="sidebar-section">
                <h3 class="section-title">{"Public Site"}</h3>
                <ul class="admin-nav">
                    <li>
                        <button 
                            class="admin-nav-link public-link"
                            onclick={on_public_click.clone()}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Home"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class="admin-nav-link public-link"
                            onclick={on_public_click.clone()}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Posts"}</span>
                        </button>
                    </li>
                </ul>
            </div>

            <div class="sidebar-section">
                <h3 class="section-title">{"Admin Panel"}</h3>
                <ul class="admin-nav">
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Dashboard { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_dashboard_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Dashboard"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Posts { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_posts_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Posts"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Pages { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_pages_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 2 2h12c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Pages"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Media { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_media_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Media"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Users { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_users_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M16 4c0-1.11.89-2 2-2s2 .89 2 2-.89 2-2 2-2-.89-2-2zm4 18v-6h2.5l-2.54-7.63A1.5 1.5 0 0 0 18.54 8H17c-.8 0-1.54.37-2.01 1l-1.7 2.26V16h-1.5v6h6zM12.5 11.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5S11 9.17 11 10s.67 1.5 1.5 1.5zM5.5 6c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2zm2 16v-7H9V9c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v6h1.5v7h4z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Users"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Comments { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_comments_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M21.99 4c0-1.1-.89-2-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4-.01-18zM18 14H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Comments"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Navigation { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_navigation_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Navigation"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Templates { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_templates_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 2 2h12c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Templates"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Analytics { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_analytics_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Analytics"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::Plugins { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_plugins_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M20.5 11H19V7c0-1.1-.9-2-2-2h-4V3.5C13 2.12 11.88 1 10.5 1S8 2.12 8 3.5V5H4c-1.1 0-2 .9-2 2v3.8h1.5c1.1 0 2 .9 2 2s-.9 2-2 2H2V20c0 1.1.9 2 2 2h3.8v-1.5c0-1.1.9-2 2-2s2 .9 2 2V22H17c1.1 0 2-.9 2-2v-4h1.5c1.1 0 2-.9 2-2s-.9-2-2-2z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Plugins"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::DesignSystem { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_design_system_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9c0-0.46-0.04-0.92-0.1-1.36c-0.98,1.37-2.58,2.26-4.4,2.26 c-2.98,0-5.4-2.42-5.4-5.4c0-1.81,0.89-3.42,2.26-4.4C12.92,3.04,12.46,3,12,3z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Design System"}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class={if props.active_tab == AdminTab::SystemSettings { "admin-nav-link active" } else { "admin-nav-link" }}
                            onclick={on_system_settings_click}
                        >
                            <span class="nav-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z"/>
                                </svg>
                            </span>
                            <span class="nav-text">{"Settings"}</span>
                        </button>
                    </li>
                </ul>
            </div>
        </nav>
    }
} 