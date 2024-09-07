
# My Rust CMS

My Rust CMS is a scalable and modular content management system (CMS) built in Rust. It uses modern web technologies like **Axum** for the backend and **Yew** for the frontend, providing a secure and high-performance platform for content creation. This CMS aims to offer a hybrid experience similar to WordPress and Elementor, enabling users to build and manage content visually.

## Table of Contents

1. [Features](#features)
2. [Project Structure](#project-structure)
3. [Dependencies and Libraries](#dependencies-and-libraries)
4. [Getting Started](#getting-started)
5. [Development](#development)
6. [Building and Running](#building-and-running)
7. [Contributing](#contributing)
8. [License](#license)

## Features

- **User Authentication**: JWT-based secure authentication.
- **Post Management**: Full CRUD for blog posts.
- **Media Library**: Upload and manage media files (images, videos).
- **Page Builder**: Drag-and-drop interface for page design.
- **Comments Moderation**: Manage comments on posts.
- **Settings Management**: Site configuration options.
- **Extensible**: Modular architecture for easy extension.

## Project Structure

```plaintext
my_rust_cms/
├── Cargo.toml
├── Trunk.toml
├── .env
├── .gitignore
├── README.md
├── Dockerfile
├── docker-compose.yml
├── migrations/
├── src/
│   ├── backend/
│   │   ├── controllers/
│   │   ├── models/
│   │   ├── services/
│   │   ├── repositories/
│   │   ├── config/
│   │   ├── middlewares/
│   │   ├── main.rs
│   │   └── lib.rs
│   ├── frontend/
│   │   ├── components/
│   │   ├── pages/
│   │   ├── templates/
│   │   ├── styles/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── shared/
│   │   ├── types.rs
│   │   ├── utils.rs
│   │   └── constants.rs
│   └── main.rs
├── static/
│   ├── images/
│   ├── css/
│   └── js/
└── tests/
    ├── integration_tests.rs
    └── unit_tests.rs
```

## Dependencies and Libraries

### 1. **Axum**
   - **Usage**: Replaces Warp as the backend framework for HTTP request handling, routes, and middleware.

### 2. **Diesel**
   - **Usage**: ORM for PostgreSQL interactions, providing strong type safety.

### 3. **Serde**
   - **Usage**: Serializes and deserializes Rust structures into JSON and vice versa.

### 4. **JWT (jsonwebtoken)**
   - **Usage**: Encodes/decodes JWT for secure user authentication.

### 5. **Yew**
   - **Usage**: Rust-based front-end framework that compiles to WebAssembly for a highly interactive UI.

### 6. **Wasm-Bindgen**
   - **Usage**: Bridges between Rust and JavaScript in the WebAssembly runtime.

### 7. **Trunk**
   - **Usage**: Bundles the WebAssembly build for the frontend.

### 8. **Dotenv**
   - **Usage**: Manages environment variables securely.

### 9. **Async-Trait**
   - **Usage**: Supports asynchronous methods in traits.

### 10. **Bcrypt**
   - **Usage**: Password hashing for secure user management.

### 11. **Thiserror**
   - **Usage**: Provides error handling throughout the project.

## Getting Started

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/).
- **PostgreSQL**: Install PostgreSQL for database setup.
- **Docker**: (Optional) Docker support is available for development and production.

### Setup

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

### Running the Project

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

### Using Docker

If you prefer Docker:

1. **Build and Run Containers**:
   ```bash
   docker-compose up --build
   ```

## Development

### Testing

- **Unit Tests**:
  ```bash
  cargo test --lib
  ```

- **Integration Tests**:
  ```bash
  cargo test --test integration_tests
  ```

## Contributing

Contributions are welcome! Please open issues or pull requests if you find bugs or want to add features. Follow the contributing guidelines for formatting and testing.

## License

This project is licensed under the MIT License.
