use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendVerificationCodeReq {
    pub to: String,
}
