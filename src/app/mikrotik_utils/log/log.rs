use mikrotik::{Client, ClientError};
use mikrotik_utils::log::types::RouterLog;

use crate::app::mikrotik_utils;

const BASE: &str = "/rest";

pub async fn logs(client: &mut Client) -> Result<Vec<RouterLog>, ClientError> {
    let url = format!("{}/log", BASE);
    client.execute_get::<Vec<RouterLog>>(&url).await
}
