## What is this?

The project is a full-stack Rust website.

* Backend - Axum

* frontend - Yew

## Table of Contents

1. [Setup Local Environment](#how-to-setup-local-environment-linux-based)
2. [Creating base project tree from the ground](#creating-base-project-tree-from-the-ground-linux-based)
3. [Fresh run](#fresh-run-linux-based)
___



## How to setup local environment (Linux based)

**REQUIRED**

Programs to be installed:

* **rustup** - in case you don't have a Rust (lang) & Cargo (package manager).
* **docker** - containerize of DB.
* **postgresql** - to be allowed to send and read queries from local pc (just testing means).

Addons to be installed:

* **sea-orm-cli** - database api generator (migrations, queries).
* **trunk** - runs local server(frontend) with autobuild.
* **wasm-bindgen-cli** - allows us to use wasm in rust.
* **wasm32-unknown-unknown** - allows us to compile to wasm.

```bash
cargo install sea-orm-cli \
    && cargo install trunk \
    && cargo install wasm-bindgen-cli \
    && rustup target add wasm32-unknown-unknown
```

**(OPTIONAL)**

* **cargo-watch** - code autoruner on save

```bash
cargo install cargo-watch
```
___



## Creating base project tree from the ground (Linux based)

[Back to top](#table-of-contents)

```bash
cargo new --bin server --vcs none \
    && cargo new --bin frontend --vcs none \
    && cargo new --lib shared --vcs none \
    && sea-orm-cli migrate init \
    && echo "[workspace]\nmembers = [\"server\", \"frontend\", \
    \"shared\", \"migration\"]" | cat > Cargo.toml \
    && echo "/target\n/dist" | cat > .gitignore
```


#### (bellow is the example of `docker-compose.yaml` file)

```bash
version: '3.8'
services:
  database:
    container_name: online_shop
    image: postgres
    restart: always
    environment:
      - POSTGRES_USER=cc0
      - POSTGRES_PASSWORD=project
      - POSTGRES_DB=website_db
    # volumes:
      # ./scripts/database/init.sql:/docker-entrypoint-initdb.d/init.sql
      # ./src/database/init.sql:/var/lib/postgresql/data/init.sql.
    healthcheck:
      test: psql -U cc0 -q -d website_db -c "SELECT 'ready';"
      interval: 10s
      timeout: 5s
      retries: 5
      start_period: 5s
    ports:
      - "5490:5432"
```


#### (bellow is the example of `.env` file)

```bash
# SUPER SECRET
DATABASE_URL="postgresql://cc0:project@localhost:5490/website_db"
JWT_SECRET=wonderful

# SERVER(backend) SIDE
SERVER_ADDR="0.0.0.0"
SERVER_PORT="5000"
SERVER_LOG_LEVEL="debug"
```

___


## Fresh run (Linux based)

[Back to top](#table-of-contents)

All commands are being executed in the root of the project

* first terminal for database startup (creates docker container and autorun it)
    ```bash
    docker-compose up
    ```

* second terminal for backend startup
    ```bash
    cargo-watch -q -c -w server/src +x run
    ```

* third terminal for frontend startup
    ```bash
    trunk serve
    ```
