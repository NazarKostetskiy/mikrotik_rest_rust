pub mod log;
pub mod system;

use actix_web::Error;
use paperclip::actix::{api_v2_operation, get, web::Json, Apiv2Schema};
use serde::{Deserialize, Serialize};

// Mark containers (body, query, parameter, etc.) like so...
#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Status {
    status: String,
}

#[api_v2_operation]
#[get("/")]
pub async fn default_endpoint() -> Result<Json<Status>, Error> {
    let obj = Status {
        status: "ok".to_string(),
    };
    Ok(Json(obj))
}
