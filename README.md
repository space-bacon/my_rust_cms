
# My Rust CMS

My Rust CMS is a powerful, flexible, and scalable content management system (CMS) built using Rust. It leverages modern web technologies such as WARP for the backend and Yew for the frontend, offering a performant and secure platform for content creation and management. The CMS is designed to provide a hybrid of WordPress and Elementor, allowing users to create and manage content via a web builder interface with drag-and-drop capabilities.

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

- **User Authentication:** Secure user authentication with JWT tokens.
- **Post Management:** Create, read, update, and delete (CRUD) operations for blog posts.
- **Media Library:** Upload and manage media files such as images and videos.
- **Page Builder:** A drag-and-drop web builder interface similar to Elementor, allowing users to design pages visually.
- **Comments Moderation:** Manage and moderate user comments on posts.
- **Settings Management:** Configure various site settings through an intuitive interface.
- **Extensible and Modular:** Built with a modular architecture, allowing for easy extension and customization.

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

### 1. **WARP**
   - **Version:** `*`
   - **Usage:** WARP is a powerful and fast web framework for Rust. It is used to build the backend server, handling HTTP requests and routing. 

### 2. **Diesel**
   - **Version:** `2.0`
   - **Features:** `postgres`
   - **Usage:** Diesel is a Safe, Extensible ORM and Query Builder for Rust. It is used for interacting with the PostgreSQL database, allowing for seamless query generation and execution. Diesel provides strong type safety for database interactions, reducing the likelihood of runtime errors.

### 3. **Serde**
   - **Version:** `1.0`
   - **Features:** `derive`
   - **Usage:** Serde is a framework for serializing and deserializing Rust data structures. It is used to convert Rust structs into JSON and vice versa, facilitating data exchange between the frontend and backend.

### 4. **JWT (jsonwebtoken)**
   - **Version:** `8.1`
   - **Usage:** The `jsonwebtoken` crate is used for encoding and decoding JSON Web Tokens (JWT). It provides secure authentication mechanisms by issuing tokens that are used to authorize API requests.

### 5. **Yew**
   - **Version:** `0.20`
   - **Features:** `csr`
   - **Usage:** Yew is a modern Rust framework for building multi-threaded front-end web apps using WebAssembly. It is used for the frontend of My Rust CMS, enabling a highly interactive user interface with rich user experiences.

### 6. **Wasm-Bindgen**
   - **Version:** `0.2`
   - **Usage:** Wasm-bindgen facilitates high-level interactions between Rust and JavaScript in WebAssembly applications. It is used in conjunction with Yew to compile Rust code to WebAssembly, which is then run in the browser.

### 7. **Trunk**
   - **Version:** `0.15.0`
   - **Usage:** Trunk is a build tool for Rust Wasm applications. It manages the build process for the Yew frontend, ensuring that the WebAssembly and JavaScript bundles are created and served correctly.

### 8. **Dotenv**
   - **Version:** `0.15`
   - **Usage:** Dotenv is used to manage environment variables. It loads environment variables from a `.env` file into the application, making it easy to configure the application without hardcoding sensitive information.

### 9. **Async-Trait**
   - **Version:** `0.1`
   - **Usage:** The `async-trait` crate is used to define asynchronous methods in trait definitions. This is particularly useful in the service layer, where asynchronous database operations are performed.

### 10. **Bcrypt**
   - **Version:** `0.9`
   - **Usage:** Bcrypt is used for hashing and verifying passwords securely. It provides a cryptographic hashing function that is computationally intensive, making it resistant to brute-force attacks.

### 11. **Thiserror**
   - **Version:** `1.0`
   - **Usage:** Thiserror provides a convenient way to define custom error types in Rust. It is used throughout the project to handle and propagate errors in a consistent and type-safe manner.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using `rustup`:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **PostgreSQL**: Install PostgreSQL for the database.
- **Docker**: (Optional) If you prefer to use Docker for development, ensure Docker is installed.

### Setup

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/space-bacon/my_rust_cms.git
   cd my_rust_cms
   ```

2. **Set Up Environment Variables:**

   Create a `.env` file in the root directory and configure your environment variables:

   ```plaintext
   DATABASE_URL=postgres://username:password@localhost/my_rust_cms
   SECRET_KEY=your_secret_key_here
   ```

3. **Run Database Migrations:**

   If you are using Diesel for database migrations, set up the database schema:

   ```bash
   diesel migration run
   ```

### Running the Project

1. **Backend:**
   - Start the backend server:
     ```bash
     cargo run
     ```
   - The backend server will be running at `http://localhost:8080`.

2. **Frontend:**
   - Start the Yew frontend with Trunk:
     ```bash
     trunk serve
     ```
   - The frontend will be served at `http://localhost:8080`.

### Using Docker (Optional)

If you prefer using Docker for development:

1. **Build and Run Containers:**
   ```bash
   docker-compose up --build
   ```
2. **Access the Application:**
   - The application will be available at `http://localhost:8080`.

## Development

### File Structure

- **Backend**:
  - `src/backend/`: Contains all backend-related code, including controllers, models, services, repositories, and configuration files.
- **Frontend**:
  - `src/frontend/`: Contains all frontend-related code, including components, pages, templates, and styles.
- **Shared**:
  - `src/shared/`: Contains shared utilities, types, and constants used across both frontend and backend.

### Running Tests

- **Unit Tests:**
  ```bash
  cargo test --lib
  ```
- **Integration Tests:**
  ```bash
  cargo test --test integration_tests
  ```

## Contributing

Contributions are welcome! If you find a bug or want to add a new feature, feel free to open an issue or submit a pull request. Please ensure that your contributions are well-documented and tested.

### How to Contribute

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

This `README.md` should provide a thorough guide to understanding, setting up, and contributing to the `my_rust_cms` project. It covers all
