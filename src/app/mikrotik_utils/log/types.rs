use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

// ".id": "*196",
// "message": "user admin logged in via api",
// "time": "16:27:43",
// "topics": "system,info,account"

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct RouterLog {
    #[serde(rename(deserialize = ".id", serialize = "id"))]
    id: Option<String>,
    message: String,
    time: String,
    topics: String,
}
