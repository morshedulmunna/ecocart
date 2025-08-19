# EcoCart API (rustdoc)

Base URL: `http://127.0.0.1:8080` (override with `API_HOST` and `API_PORT`).

This crate integrates developer-facing API docs via Rust's built-in documentation system.

## Building Docs

```bash
cargo doc --no-deps --open
```

See the rustdoc book for background: [What is rustdoc?](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)

## API Categories

- [Auth](crate::api_docs::auth)
- [Users](crate::api_docs::users)
- [Categories](crate::api_docs::categories)
- [Products](crate::api_docs::products)
- [Health](crate::api_docs::health)
- [Error Model](crate::api_docs::errors)
