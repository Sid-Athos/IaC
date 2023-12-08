use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::session::Session;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SessionResponse {
    pub token: String,
}

impl SessionResponse {
    pub fn from_domain(session: Session) -> Self {
        SessionResponse {
            token: session.token,
        }
    }
}
