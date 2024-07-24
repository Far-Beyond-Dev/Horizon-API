# Horizon API

Horizon API is a simple RESTful API built using Rust. This API allows you to create new users and fetch user information based on their username.

## Features

- **Create User**: Register a new user with a username and password.
- **Get User**: Retrieve user information by username.

## Getting Started

### Setup

1. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/horizon-api.git
   cd horizon-api
   ```

2. **Setup the database**:
   Run Diesel migrations to set up the database schema.
   ```sh
    mv ./.env.example ./.env
   ```
### Running the API

To start the Rocket server, run:
```sh
cargo run
```

The server will be running at `http://localhost:8000`.