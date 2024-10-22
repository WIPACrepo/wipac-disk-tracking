// token.rs

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use axum_extra::response::ErasedJson;
use axum_keycloak_auth::{
    decode::KeycloakToken, instance::KeycloakAuthInstance, layer::KeycloakAuthLayer,
    role::KeycloakRole, PassthroughMode,
};
use serde::Serialize;
use std::sync::Arc;
use time::OffsetDateTime;

use crate::context::ApplicationContext;
use crate::middleware::EmptyExtra;

// function to serialize `OffsetDateTime` as a Unix timestamp
fn serialize_offset_date_time_as_unix_timestamp<S>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let unix_timestamp = datetime.unix_timestamp();
    serializer.serialize_i64(unix_timestamp)
}

#[derive(Serialize)]
pub struct SerializableToken {
    /// Expiration time (UTC).
    #[serde(serialize_with = "serialize_offset_date_time_as_unix_timestamp")]
    pub expires_at: OffsetDateTime,
    /// Issued at time (UTC).
    #[serde(serialize_with = "serialize_offset_date_time_as_unix_timestamp")]
    pub issued_at: OffsetDateTime,
    /// JWT ID (unique identifier for this token).
    pub jwt_id: String,
    /// Issuer (who created and signed this token).
    pub issuer: String,
    /// Audience (who or what the token is intended for).
    pub audience: Vec<String>,
    /// Subject (whom the token refers to). This is the UUID which uniquely identifies this user inside Keycloak.
    pub subject: String,
    /// Authorized party (the party to which this token was issued).
    pub authorized_party: String,
    // Keycloak: Roles of the user.
    pub roles: Vec<KeycloakRole<String>>,
    // addtional fields (actually, no additional fields)
    pub extra: EmptyExtra,
}

impl From<KeycloakToken<String, EmptyExtra>> for SerializableToken {
    fn from(token: KeycloakToken<String, EmptyExtra>) -> Self {
        SerializableToken {
            expires_at: token.expires_at,
            issued_at: token.issued_at,
            jwt_id: token.jwt_id,
            issuer: token.issuer,
            audience: token.audience,
            subject: token.subject,
            authorized_party: token.authorized_party,
            roles: token.roles,
            extra: EmptyExtra,
        }
    }
}

pub fn build_router(context: ApplicationContext, instance: Arc<KeycloakAuthInstance>) -> Router {
    // save the audience
    let oauth_audience = context.oauth_audience.clone();

    // build the routes under /token
    Router::new()
        .route("/token", get(get_token))
        .with_state(context)
        .layer(
            KeycloakAuthLayer::<String, EmptyExtra>::builder()
                .instance(instance)
                .passthrough_mode(PassthroughMode::Block)
                .persist_raw_claims(false)
                .expected_audiences(vec![oauth_audience])
                .required_roles(vec![String::from("system")])
                .build(),
        )
}

pub async fn get_token(
    Extension(token): Extension<KeycloakToken<String, EmptyExtra>>,
) -> impl IntoResponse {
    let output_token: SerializableToken = token.into();
    (StatusCode::OK, ErasedJson::pretty(output_token))
}
