# Example of using a full stack with Rust.

### This repository is an example of a complete stack built entirely in Rust ("RASTY"):

- [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)) as almost unique base language (except for minimal JS needed to style the interface with TailwindCSS)
- [Actix-web](https://actix.rs/), a Rust framework, for the backend (our API server)
- [SurrealDB](https://surrealdb.com/), the database that connects with our API, which has been developed by its creators with Rust
- [Yew.rs](https://yew.rs/), Rust's web framework for building web applications with Webassembly

### In itself, the frontend applications perform a CRUD to the database using the RESTful API.

---

## To run the application stack

### 1.- actix-surrealdb-api, server/API built using Rust's Actix-web framework and SurrealDB database running in a Docker container.

In addition to the obvious prerequisite of having Rust and Docker installed, we need to do the following:

Create the container with a SurrealDB image and a Docker managed storage volume running at the project root:

```
$ docker compose up -d
```

If we have _cargo-watch_ installed using:

```
$ cargo install cargo-watch
```

we won't have to restart the server every time we make a code change; running the following command in the root of the project actix-surrealdb-api:

```
$ cargo watch -x run
```

rather:

```
$ cargo run
```

the server will restart automatically 😀.

To stop the Docker container in which SurrealDB is running:

```
$ docker stop surrealdb
```

And to raise the container again:

```
$ docker start surrealdb
```

We can read the content of the database with the commands:

```
$ docker exec -it surrealdb /surreal sql -c http://localhost:8000 -u root -p root --ns namespace --db database --pretty

namespace/database> SELECT * FROM task ORDER BY created_at DESC;
```

### 2.- todo-yew-web, Web application developed with Rust/WebAssembly + Yew + Tailwindcss.

To run the Web App, add the WebAssembly target:

```
$ rustup target add wasm32-unknown-unknown
```

Install [Trunk](https://trunkrs.dev/) (web application bundler for Rust) to run the app:

```
$ cargo install trunk
```

Run the following command in the root of the project todo-yew-web:

```
$ trunk serve
```

Build the application by running:

```
$ trunk build --release
```
