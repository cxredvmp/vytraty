use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{AppState, model, route};

#[derive(OpenApi)]
#[openapi(
    paths(
        route::auth::signup,
        route::auth::signin,
        route::me::get_me,
        route::me::delete_me,
        route::categories::create,
        route::categories::get_all,
        route::categories::delete_by_id,
        route::records::create,
        route::records::get_all,
        route::records::delete_by_id,
        route::health::check
    ),
    components(
        schemas(
            model::auth::SignUp,
            model::auth::SignIn,
            model::auth::Token,
            model::user::Read,
            model::user::Body<model::user::Read>,
            model::category::CreateRequest,
            model::category::Read,
            model::category::Body<model::category::Read>,
            model::category::BodyArray<model::category::Read>,
            model::record::CreateRequest,
            model::record::Read,
            model::record::Body<model::record::Read>,
            model::record::BodyArray<model::record::Read>,
            model::health::Health,
            model::health::Services,
            model::health::Status,
        )
    ),
    tags(
        (name = "Vytraty", description = "Vytraty expense tracking API")
    ),
    modifiers(&SecurityAddon)
)]
pub struct OpenApiDoc;

struct SecurityAddon;
impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", OpenApiDoc::openapi()))
}
