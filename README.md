## EcoCart — Monorepo

Modern e-commerce playground with a Rust backend (Axum, SQLx, Redis) and a Next.js 15 frontend.

- **Backend**: `api-server/` (Rust, Axum, Postgres, Redis)
- **Frontend**: `web/` (Next.js 15, React 19, Tailwind)

### Repo structure

```
EcoCart/
  api-server/      # Rust API (Axum), Docker, SQLx migrations, docs
  web/             # Next.js frontend
```

## Prerequisites

- Docker and Docker Compose
- Node.js 18+ and npm (for local frontend)
- Rust toolchain (if you want to run the backend locally instead of Docker)

## Environment setup

### Backend (`api-server/.env`)

1. Copy example env:

```bash
cd api-server
cp example.env .env
```

2. Key variables (edit as needed):

```bash
# --- Database ---
DATABASE_URL=postgres://ecocart:password@localhost:5432/ecocart?sslmode=disable
DB_HOST=localhost
DB_PORT=5432
DB_USER=ecocart
DB_PASSWORD=password
DB_NAME=ecocart
DB_SSLMODE=disable

# --- System ---
APP_ENV=development
LOG_LEVEL=info
API_HOST=127.0.0.1
API_PORT=8080
GRPC_HOST=127.0.0.1
GRPC_PORT=50051
SHUTDOWN_GRACE_SECONDS=10

# --- Auth/JWT ---
JWT_SECRET=change-me-in-production
JWT_ISSUER=ecocart
JWT_ACCESS_TTL_SECONDS=900
JWT_REFRESH_TTL_SECONDS=2592000

# --- Redis ---
REDIS_URL=redis://127.0.0.1:6379/0
REDIS_HOST=127.0.0.1
REDIS_PORT=6379
REDIS_USERNAME=
REDIS_PASSWORD=
REDIS_DB=0
REDIS_TLS=false

# --- Seed (Admin) ---
# Used by the `ecocart seed` command to create/promote an admin
ADMIN_USERNAME=admin
ADMIN_EMAIL=admin@ecocart.local
ADMIN_PASSWORD=admin123
```

Notes:

- Docker Compose overrides some values (e.g., `API_HOST=0.0.0.0`, DB/Redis hosts set to service names inside the network).
- Change `JWT_SECRET` for any non-local usage.

### Frontend (`web/.env.local`)

Create an env file that points the app to the backend:

```bash
cd web
printf "API_BASE_URL=http://127.0.0.1:8080\n" > .env.local
```

This is read in `web/src/lib/config.ts`.

## Run with Docker (backend + frontend)

From the repo root (Compose file lives here):

```bash
# 1) Start Postgres, Redis, API, and Web (builds images on first run)
docker compose up --build -d

# 2) Tail API logs
docker compose logs -f api

# 3) Stop stack
docker compose down
```

- Web app: `http://localhost:3000`
- API base URL: `http://localhost:8080`
- Health check: `GET http://localhost:8080/api/health`
- Static uploads: `http://localhost:8080/uploads/...`
- Swagger UI: `http://localhost:8080/docs`
- OpenAPI JSON: `http://localhost:8080/api-docs/openapi.json`

In this setup, the API container runs migrations and seeds a default admin before starting the server.

## Admin access (seeded user)

The admin account is created by the seed command using values from `api-server/example.env` (or your `.env`):

```bash
ADMIN_USERNAME=admin
ADMIN_EMAIL=admin@ecocart.local
ADMIN_PASSWORD=admin123
```

Ways to access admin:

- Frontend UI:

  - Visit `http://localhost:3000/auth/login`
  - Log in using the admin email/password above
  - Then navigate to `http://localhost:3000/admin`

- API directly:
  - `POST http://localhost:8080/api/auth/login`
  - JSON body:
    ```json
    { "email": "admin@ecocart.local", "password": "admin123" }
    ```
  - Use the returned `access_token` as `Authorization: Bearer <token>` for admin-only endpoints (e.g., creating categories).

To seed manually when running locally, run:

```bash
cd api-server
cargo run -- seed
```

The seed is idempotent: if the user exists it will be promoted to admin and the password rotated; otherwise it will be created.

## Database migrations

You have two options:

- Recommended (sqlx-cli):

```bash
cd api-server
cargo install sqlx-cli --no-default-features --features rustls,postgres
DATABASE_URL=postgres://ecocart:password@localhost:5432/ecocart?sslmode=disable \
  sqlx migrate run
```

- Project command:

```bash
cd api-server
DATABASE_URL=postgres://ecocart:password@localhost:5432/ecocart?sslmode=disable \
  make migrate
```

Tip: When the stack is up via Docker, Postgres is exposed on `localhost:5432`, so you can run migrations from your host using the provided `DATABASE_URL`.

## Run the backend locally (without Docker)

```bash
cd api-server

# 1) Run migrations (DATABASE_URL must point to your Postgres)
cargo run -- migrate

# 2) Seed admin (uses ADMIN_* from .env)
cargo run -- seed

# 3) Start the API server
cargo run -- ecocart-apis

# Or hot-reload (requires cargo-watch)
make dev
```

Useful targets (`api-server/Makefile`): `make run`, `make test`, `make fmt`, `make clippy`, `make docs`.

## View backend docs

Rust API docs are generated from Markdown in `api-server/docs/` and embedded via rustdoc.

```bash
cd api-server
make docs          # runs: cargo doc --no-deps --open
```

The docs will open in your browser (`target/doc/ecocart/index.html`).

## Swagger / OpenAPI

- Swagger UI: `http://127.0.0.1:8080/docs`
- Raw OpenAPI JSON: `http://127.0.0.1:8080/api-docs/openapi.json`

The API uses `utoipa` to generate OpenAPI schemas and `utoipa-swagger-ui` to serve the interactive docs.

How to document endpoints:

1. Derive schemas for request/response types using `ToSchema` (and query params with `IntoParams`):

```rust
use utoipa::{ToSchema, IntoParams};

#[derive(serde::Deserialize, ToSchema)]
pub struct CreateThing { name: String }

#[derive(serde::Serialize, ToSchema)]
pub struct Thing { id: i32, name: String }

#[derive(serde::Deserialize, IntoParams)]
pub struct ThingQuery { page: Option<i64> }
```

2. Annotate handlers with `#[utoipa::path(...)]`:

```rust
#[utoipa::path(
    post,
    path = "/api/things",
    request_body = CreateThing,
    responses((status = 200, body = Thing)),
    tag = "Things"
)]
async fn create_thing(Json(req): Json<CreateThing>) -> AppResult<Json<Thing>> { /* ... */ }
```

3. Add new paths or schemas to the central OpenAPI in `api-server/src/interfaces/http/swagger.rs` under the `#[openapi(paths(...), components(schemas(...)))]` macro if they are not auto-discovered.

Notes:

- Auth-protected endpoints can declare security via `security(("bearerAuth" = []))` in the `#[utoipa::path]` macro.
- Common error payload: `ApiErrorResponse` (already exported to the OpenAPI components).

## Run the frontend

```bash
cd web
npm ci
npm run dev
# App: http://localhost:3000
```

Build and start production server:

```bash
cd web
npm run build
npm start
```

Ensure `web/.env.local` contains:

```bash
API_BASE_URL=http://127.0.0.1:8080
```

## API endpoints

- Root & health:
  - `GET /` → API root
  - `GET /api/health` → health probe
- Auth:
  - `POST /api/auth/register`
  - `POST /api/auth/login`
  - `POST /api/auth/refresh`
  - `POST /api/auth/logout`
- Users:
  - `GET /api/users/:id`
- Categories:
  - `GET /api/categories`
  - `POST /api/categories` (admin)
  - `GET /api/categories/:id`
  - `PUT /api/categories/:id` (admin)
  - `DELETE /api/categories/:id` (admin)
- Products:
  - `GET /api/products`
  - `POST /api/products`
  - `GET /api/products/:id`
  - `PUT /api/products/:id`
  - `DELETE /api/products/:id`
  - `POST /api/products/upload` → multipart upload; responses return `/uploads/<file>` URL
- Static files:
  - `GET /uploads/*` → static file server for uploaded assets

## Notes & troubleshooting

- CORS is permissive in dev; rate limiting defaults to ~100 requests per 60 seconds.
- If the API container starts before Postgres/Redis are healthy, Compose will delay the API until they are ready.
- If migrations fail, verify `DATABASE_URL` and that Postgres is reachable (`docker compose ps` and `docker compose logs postgres`).
- On Apple Silicon, Docker images are multi-arch; no special flags should be required.

## Scripts quick reference

### Backend

- `make dev` — run with hot-reload
- `make run` — run the selected subcommand (`CMD`), default is `ecocart-apis`
- `make migrate` — run SQLx migrations (requires `DATABASE_URL`)
- `make test` / `make fmt` / `make clippy`
- `make docs` — build and open rustdoc

### Frontend

- `npm run dev` — local dev server (Next.js)
- `npm run build` — production build
- `npm start` — start production server

---

Happy Coding!

Morshedul Islam Munna
Software Engineer
Email: morshedulmunna1@gmail.com
