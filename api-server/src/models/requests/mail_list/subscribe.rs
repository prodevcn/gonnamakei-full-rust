use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use commons::error::AppResult;
use commons::server::validators::email_validator;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailListSubscribeRequestBody {
    pub email: ArcStr,
}

impl MailListSubscribeRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        email_validator(self.email.as_str(), "email")?;

        Ok(())
    }
}
