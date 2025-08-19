use utoipa::OpenApi;

/// Central OpenAPI document for EcoCart HTTP API.
///
/// This aggregates all annotated `#[utoipa::path]` endpoints and schemas.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "EcoCart API",
        version = "0.1.0",
        description = "REST API for EcoCart. See category, product, auth, and user endpoints."
    ),
    servers(
        (url = "http://localhost:8080", description = "Local dev")
    ),
    paths(
        crate::interfaces::http::routes::auth::register,
        crate::interfaces::http::routes::auth::login,
        crate::interfaces::http::routes::auth::refresh,
        crate::interfaces::http::routes::auth::logout,
        crate::interfaces::http::routes::users::get_user,
        crate::interfaces::http::routes::categories::list_categories,
        crate::interfaces::http::routes::categories::create_category,
        crate::interfaces::http::routes::categories::get_category,
        crate::interfaces::http::routes::categories::update_category,
        crate::interfaces::http::routes::categories::delete_category,
        crate::interfaces::http::routes::products::list_products,
        crate::interfaces::http::routes::products::create_product,
        crate::interfaces::http::routes::products::get_product,
        crate::interfaces::http::routes::products::update_product,
        crate::interfaces::http::routes::products::delete_product,
        crate::interfaces::http::routes::products::upload_image,
    ),
    components(schemas(
        crate::pkg::response::ApiErrorResponse,
        crate::interfaces::http::routes::users::UserProfile,
        crate::interfaces::http::routes::categories::CategoryDto,
        crate::interfaces::http::routes::categories::CreateCategory,
        crate::interfaces::http::routes::categories::UpdateCategory,
        crate::interfaces::http::routes::products::ProductDto,
        crate::interfaces::http::routes::products::CreateProduct,
        crate::interfaces::http::routes::products::UpdateProduct,
        crate::interfaces::http::routes::products::PaginatedProducts,
        crate::interfaces::http::routes::auth::RegisterRequest,
        crate::interfaces::http::routes::auth::RegisterResponse,
        crate::interfaces::http::routes::auth::LoginRequest,
        crate::interfaces::http::routes::auth::UserResponse,
        crate::interfaces::http::routes::auth::LoginResponse,
        crate::interfaces::http::routes::auth::TokenResponse,
        crate::interfaces::http::routes::auth::RefreshRequest,
        crate::interfaces::http::routes::auth::OkResponse,
    ))
)]
pub struct ApiDoc;
