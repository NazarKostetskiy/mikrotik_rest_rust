use actix_web::Error;
use mikrotik::Client;
use paperclip::actix::{api_v2_operation, get, web::Json, Apiv2Schema};
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::app::mikrotik_utils::{
    log::{log::logs, types::RouterLog},
    PASSWORD, ROUTER_URL, USERNAME,
};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Logs {
    data: Vec<RouterLog>,
}

#[api_v2_operation]
#[get("/logs")]
pub async fn get_logs() -> Result<Json<Logs>, Error> {
    let mut mikrotik_client = Client::new(
        Url::parse(ROUTER_URL).unwrap(),
        String::from(USERNAME),
        String::from(PASSWORD),
        true,
    )
    .unwrap();

    let log_data = logs(&mut mikrotik_client).await.unwrap(); // TODO: Use unwrap error here later

    let obj = Logs { data: log_data };
    Ok(Json(obj))
}
