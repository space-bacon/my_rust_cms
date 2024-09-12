
# My Rust CMS 🦀🖥️

My Rust CMS is a scalable and modular content management system (CMS) built in Rust. It uses modern web technologies like **Axum** for the backend and **Yew** for the frontend, providing a secure and high-performance platform for content creation. This CMS offers a hybrid experience similar to WordPress and Elementor, enabling users to build and manage content visually. 🚀

## Table of Contents 📑

1. [Features](#features)
2. [Project Structure](#project-structure)
3. [Frontend User Interface](#frontend-user-interface)
4. [Dependencies and Libraries](#dependencies-and-libraries)
5. [Getting Started](#getting-started)
6. [Development](#development)
7. [Building and Running](#building-and-running)
8. [Contributing](#contributing)
9. [License](#license)

## Features ✨

- **User Authentication**: JWT-based secure authentication 🔐.
- **Post Management**: Full CRUD for blog posts 📝.
- **Media Library**: Upload and manage media files (images, videos) 📂.
- **Page Builder**: Drag-and-drop interface for page design 🎨.
- **Comments Moderation**: Manage comments on posts 💬.
- **Settings Management**: Site configuration options ⚙️.
- **Extensible**: Modular architecture for easy extension 🔧.

## Project Structure 🗂️

```plaintext
my_rust_cms/
│
├── backend/
│   ├── src/
│   │   ├── controllers/
│   │   │   ├── mod.rs                      # Re-exports all controllers
│   │   │   ├── auth_controller.rs           # Handles user login, signup, etc.
│   │   │   ├── post_controller.rs           # Handles post CRUD operations
│   │   │   ├── media_controller.rs          # Handles media management
│   │   │   ├── category_controller.rs       # Handles categories for posts
│   │   │   ├── builder_controller.rs        # Manages page builder content
│   │   │   ├── settings_controller.rs       # Manages CMS settings
│   │   ├── models/
│   │   │   ├── mod.rs                      # Re-exports all models
│   │   │   ├── user.rs                     # User data model
│   │   │   ├── post.rs                     # Post data model
│   │   │   ├── media.rs                    # Media data model
│   │   │   ├── category.rs                 # Category model
│   │   │   ├── settings.rs                 # CMS settings model
│   │   │   ├── builder.rs                  # Page builder model
│   │   ├── views/
│   │   │   ├── mod.rs                      # Re-exports all views
│   │   │   ├── auth_view.rs                # Authentication-related views (login, signup, etc.)
│   │   │   ├── post_view.rs                # Post views (create, edit, delete)
│   │   │   ├── media_view.rs               # Media views (upload, delete)
│   │   │   ├── category_view.rs            # Category views (add, delete, edit)
│   │   │   ├── settings_view.rs            # Settings views (edit settings)
│   │   │   ├── builder_view.rs             # Builder view for page creation
│   │   ├── utils/
│   │   │   ├── mod.rs                      # Re-exports all utility modules
│   │   │   ├── db.rs                       # Database connection and pooling
│   │   │   ├── errors.rs                   # Error handling utilities
│   │   │   ├── validation.rs               # Input validation helpers
│   │   │   ├── auth.rs                     # Authentication-related utilities (JWT, hashing, etc.)
│   │   ├── schema.rs                       # Diesel schema generated for database tables
│   │   ├── main.rs                         # Entry point for the backend application
│   ├── Cargo.toml                          # Rust dependencies and configurations for backend
│   ├── Diesel.toml                         # Diesel setup for managing migrations
│   └── .env                                # Environment variables (DB URL, JWT secret, etc.)
│
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── mod.rs                      # Re-exports all components
│   │   │   ├── editor.rs                   # Post/page editor component
│   │   │   ├── media_manager.rs            # Media manager component (upload, delete)
│   │   │   ├── sidebar.rs                  # Sidebar with navigation (posts, media, settings)
│   │   │   ├── tab_view.rs                 # Tabbed interface for post/media editing
│   │   │   ├── category_manager.rs         # Category manager component
│   │   │   ├── settings_panel.rs           # Settings panel component
│   │   ├── pages/
│   │   │   ├── mod.rs                      # Re-exports all pages
│   │   │   ├── login.rs                    # Login page component
│   │   │   ├── dashboard.rs                # Main dashboard page after login
│   │   │   ├── post_editor.rs              # Page for editing or creating posts
│   │   │   ├── media_dashboard.rs          # Media management dashboard
│   │   │   ├── settings_page.rs            # Settings page
│   │   ├── app.rs                          # Main app component
│   │   ├── main.rs                         # Entry point for the frontend application
│   ├── static/
│   │   ├── css/
│   │   │   ├── style.css                   # Custom styles for the CMS frontend
│   │   ├── images/
│   │   │   ├── logo.png                    # CMS logo
│   │   ├── js/
│   │   │   ├── custom.js                   # Any custom JS if needed
│   ├── Trunk.toml                          # Trunk config for building the frontend
│   ├── index.html                          # Frontend entry point for the browser
│   └── assets/
│       ├── favicon.ico                     # Favicon for the frontend
│       ├── manifest.json                   # Web app manifest
│
├── migrations/
│   ├── 2024-09-10-120000_create_users_table/
│   │   ├── up.sql                          # SQL to create users table
│   │   ├── down.sql                        # SQL to drop users table
│   ├── 2024-09-10-120100_create_posts_table/
│   │   ├── up.sql                          # SQL to create posts table
│   │   ├── down.sql                        # SQL to drop posts table
│   ├── 2024-09-10-120200_create_media_table/
│   │   ├── up.sql                          # SQL to create media table
│   │   ├── down.sql                        # SQL to drop media table
│   ├── 2024-09-10-120300_create_categories_table/
│   │   ├── up.sql                          # SQL to create categories table
│   │   ├── down.sql                        # SQL to drop categories table
│   ├── 2024-09-10-120400_create_builder_table/
│   │   ├── up.sql                          # SQL to create builder table for page builder
│   │   ├── down.sql                        # SQL to drop builder table
│   ├── 2024-09-10-120500_create_settings_table/
│   │   ├── up.sql                          # SQL to create settings table for CMS configuration
│   │   ├── down.sql                        # SQL to drop settings table
│
├── config/
│   ├── dev.toml                            # Development environment configuration
│   ├── prod.toml                           # Production environment configuration
│
├── tests/
│   ├── integration_tests.rs                # Integration tests for various controllers
│   ├── unit_tests.rs                       # Unit tests for utility functions, models
│
├── scripts/
│   ├── build_frontend.sh                   # Script to build the frontend using Trunk
│   ├── run_migrations.sh                   # Script to run Diesel migrations
│   ├── start_dev_backend.sh                # Script to run the backend in development mode
│
├── .gitignore                              # Files to ignore in Git
├── README.md                               # Project documentation
├── LICENSE                                 # License information
└── Dockerfile                              # Docker configuration for the project
```

## Frontend User Interface 🎨

The **My Rust CMS** frontend interface is designed to resemble **Visual Studio Code** (VS Code) for an intuitive and familiar user experience. The layout features:

- **Slim Vertical Sidebar**: Located on the left-hand side, this sidebar houses icons representing various key sections such as **Posts**, **Media**, and **Settings**. These icons make it

 easy to navigate between the major content management features.

- **Explorer Panel**: When clicking on an icon, an **Explorer** panel slides out from the left, displaying content such as posts and categories. This is organized similarly to a file explorer, where categories are shown as folders and posts as file types, making navigation seamless.

- **Tabbed Interface**: Upon selecting a post or media item, it opens in a new tab within the main content area. This mimics the tab system of VS Code, where multiple items can be edited simultaneously.

- **Editor and Components**: The right side of the interface consists of the post editor or media manager, allowing users to work on content while having easy access to other posts or media files in the tabs. This dynamic and modern interface allows for flexibility in content creation and management.

Overall, the user interface is streamlined for developers and content creators alike, combining the familiarity of VS Code’s design with powerful content management tools.

---

## Dependencies and Libraries 📦

### 1. **Axum**
   - **Usage**: Replaces Warp as the backend framework for HTTP request handling, routes, and middleware.

### 2. **Diesel**
   - **Usage**: ORM for PostgreSQL interactions, providing strong type safety.

### 3. **Serde**
   - **Usage**: Serializes and deserializes Rust structures into JSON and vice versa 🔄.

### 4. **JWT (jsonwebtoken)**
   - **Usage**: Encodes/decodes JWT for secure user authentication 🔑.

### 5. **Yew**
   - **Usage**: Rust-based front-end framework that compiles to WebAssembly for a highly interactive UI 🌐.

### 6. **Wasm-Bindgen**
   - **Usage**: Bridges between Rust and JavaScript in the WebAssembly runtime 🛠️.

### 7. **Trunk**
   - **Usage**: Bundles the WebAssembly build for the frontend 🎁.

### 8. **Dotenv**
   - **Usage**: Manages environment variables securely 🛡️.

### 9. **Async-Trait**
   - **Usage**: Supports asynchronous methods in traits.

### 10. **Bcrypt**
   - **Usage**: Password hashing for secure user management 🔒.

### 11. **Thiserror**
   - **Usage**: Provides error handling throughout the project ⚠️.

---

## Getting Started 🚀

### Prerequisites 🛠️

- **Rust**: Install via [rustup](https://rustup.rs/).
- **PostgreSQL**: Install PostgreSQL for database setup 🐘.
- **Docker**: (Optional) Docker support is available for development and production 🐋.

### Setup 🔧

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/space-bacon/my_rust_cms.git
   cd my_rust_cms
   ```

2. **Set Up Environment Variables**:

   Create a `.env` file in the root directory and add your environment variables:

   ```plaintext
   DATABASE_URL=postgres://username:password@localhost/my_rust_cms
   SECRET_KEY=your_secret_key_here
   ```

3. **Run Migrations**:

   ```bash
   diesel migration run
   ```

### Running the Project ▶️

1. **Backend**:
   - Start the backend:
     ```bash
     cargo run
     ```

2. **Frontend**:
   - Run the frontend using Trunk:
     ```bash
     trunk serve
     ```

### Using Docker 🐳

If you prefer Docker:

1. **Build and Run Containers**:
   ```bash
   docker-compose up --build
   ```

## Development 🧑‍💻

### Testing 🧪

- **Unit Tests**:
  ```bash
  cargo test --lib
  ```

- **Integration Tests**:
  ```bash
  cargo test --test integration_tests
  ```

## Contributing 🤝

Contributions are welcome! Please open issues or pull requests if you find bugs or want to add features. Follow the contributing guidelines for formatting and testing.

## License 📄

This project is licensed under the MIT License.
```
