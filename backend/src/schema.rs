// @generated automatically by Diesel CLI.

diesel::table! {
    backup_logs (id) {
        id -> Uuid,
        backup_id -> Nullable<Uuid>,
        schedule_id -> Nullable<Uuid>,
        level -> Varchar,
        message -> Text,
        details -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backup_schedules (id) {
        id -> Uuid,
        name -> Varchar,
        backup_type -> Varchar,
        cron_expression -> Varchar,
        is_active -> Nullable<Bool>,
        retention_days -> Nullable<Int4>,
        max_backups -> Nullable<Int4>,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_run_at -> Nullable<Timestamptz>,
        next_run_at -> Nullable<Timestamptz>,
        created_by -> Nullable<Uuid>,
        settings -> Nullable<Jsonb>,
    }
}

diesel::table! {
    backups (id) {
        id -> Uuid,
        filename -> Varchar,
        backup_type -> Varchar,
        status -> Varchar,
        file_size -> Nullable<Int8>,
        checksum -> Nullable<Varchar>,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        created_by -> Nullable<Uuid>,
        metadata -> Nullable<Jsonb>,
        error_message -> Nullable<Text>,
        retention_policy -> Nullable<Varchar>,
        is_encrypted -> Nullable<Bool>,
        compression_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    builder_components (id) {
        id -> Int4,
        component_name -> Varchar,
        component_data -> Nullable<Jsonb>,
        template_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        page_id -> Nullable<Int4>,
    }
}

diesel::table! {
    component_events (id) {
        id -> Int4,
        component_id -> Nullable<Int4>,
        event_type -> Varchar,
        event_handler -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    component_styles (id) {
        id -> Int4,
        component_id -> Nullable<Int4>,
        css -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    component_templates (id) {
        id -> Int4,
        name -> Varchar,
        component_type -> Varchar,
        template_data -> Jsonb,
        breakpoints -> Jsonb,
        width_setting -> Nullable<Varchar>,
        max_width -> Nullable<Varchar>,
        is_default -> Bool,
        is_active -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    components (id) {
        id -> Int4,
        name -> Varchar,
        template_id -> Nullable<Int4>,
        component_data -> Jsonb,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    media (id) {
        id -> Int4,
        file_name -> Varchar,
        url -> Varchar,
        media_type -> Nullable<Varchar>,
        uploaded_at -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    menu_areas (id) {
        id -> Int4,
        area_name -> Varchar,
        display_name -> Varchar,
        template_id -> Nullable<Int4>,
        settings -> Jsonb,
        mobile_behavior -> Nullable<Varchar>,
        hamburger_icon -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    menu_templates (id) {
        id -> Int4,
        name -> Varchar,
        template_type -> Varchar,
        layout_style -> Varchar,
        settings -> Jsonb,
        is_active -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    navigation (id) {
        id -> Int4,
        title -> Varchar,
        url -> Varchar,
        order_position -> Int4,
        is_active -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        menu_area -> Varchar,
        parent_id -> Nullable<Int4>,
        icon -> Nullable<Varchar>,
        css_class -> Nullable<Varchar>,
        target -> Nullable<Varchar>,
        mobile_visible -> Bool,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    page_components (id) {
        id -> Int4,
        page_id -> Nullable<Int4>,
        component_id -> Nullable<Int4>,
        position -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    page_sections (id) {
        id -> Int4,
        page_id -> Nullable<Int4>,
        section_name -> Varchar,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pages (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        slug -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    plugin_hooks (id) {
        id -> Int4,
        plugin_id -> Int4,
        hook_name -> Varchar,
        priority -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    plugin_settings (id) {
        id -> Int4,
        plugin_id -> Int4,
        setting_key -> Varchar,
        setting_value -> Nullable<Jsonb>,
        setting_type -> Nullable<Varchar>,
        is_encrypted -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    plugins (id) {
        id -> Int4,
        name -> Varchar,
        display_name -> Varchar,
        description -> Nullable<Text>,
        version -> Varchar,
        author -> Nullable<Varchar>,
        author_email -> Nullable<Varchar>,
        homepage_url -> Nullable<Varchar>,
        repository_url -> Nullable<Varchar>,
        license -> Nullable<Varchar>,
        status -> Varchar,
        is_system -> Bool,
        install_path -> Nullable<Varchar>,
        config_schema -> Nullable<Jsonb>,
        config_data -> Nullable<Jsonb>,
        capabilities -> Nullable<Jsonb>,
        dependencies -> Nullable<Jsonb>,
        installed_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        last_activated_at -> Nullable<Timestamp>,
        activation_count -> Nullable<Int4>,
        last_error -> Nullable<Text>,
        error_count -> Nullable<Int4>,
        manifest_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        category_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        session_token -> Varchar,
        created_at -> Nullable<Timestamp>,
        expires_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        setting_key -> Varchar,
        setting_value -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        setting_type -> Varchar,
        description -> Nullable<Text>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    templates (id) {
        id -> Int4,
        name -> Varchar,
        layout -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        role -> Varchar,
        status -> Varchar,
        email_verified -> Bool,
        email_verification_token -> Nullable<Varchar>,
        email_verification_expires_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(backup_logs -> backup_schedules (schedule_id));
diesel::joinable!(backup_logs -> backups (backup_id));
diesel::joinable!(builder_components -> templates (template_id));
diesel::joinable!(comments -> pages (page_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(component_events -> components (component_id));
diesel::joinable!(component_styles -> components (component_id));
diesel::joinable!(components -> templates (template_id));
diesel::joinable!(media -> users (user_id));
diesel::joinable!(menu_areas -> menu_templates (template_id));
diesel::joinable!(page_components -> components (component_id));
diesel::joinable!(page_components -> pages (page_id));
diesel::joinable!(page_sections -> pages (page_id));
diesel::joinable!(pages -> users (user_id));
diesel::joinable!(plugin_hooks -> plugins (plugin_id));
diesel::joinable!(plugin_settings -> plugins (plugin_id));
diesel::joinable!(posts -> categories (category_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    backup_logs,
    backup_schedules,
    backups,
    builder_components,
    categories,
    comments,
    component_events,
    component_styles,
    component_templates,
    components,
    media,
    menu_areas,
    menu_templates,
    navigation,
    page_components,
    page_sections,
    pages,
    plugin_hooks,
    plugin_settings,
    plugins,
    posts,
    sessions,
    settings,
    templates,
    users,
);
