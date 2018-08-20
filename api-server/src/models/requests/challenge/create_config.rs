use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use commons::constants::MAX_REWARD_MULTIPLIER;
use commons::database::documents::APIChallengeMilestone;
use commons::error::{AppError, AppResult, INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE};
use commons::server::validators::string_length_validator;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeCreateConfigRequestBody {
    pub name: ArcStr,
    pub description: ArcStr,
    pub milestones: Vec<APIChallengeMilestone>,
    pub max_bet: Option<u64>,
    pub reward_multiplier: u64,
}

impl ChallengeCreateConfigRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        string_length_validator(self.name.as_str(), 4, 30, "name")?;
        string_length_validator(self.description.as_str(), 0, 255, "description")?;

        if self.milestones.is_empty() {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
            )
            .message(arcstr::literal!("Empty milestones list"))
            .param(arcstr::literal!("milestones")));
        }

        if self.reward_multiplier > MAX_REWARD_MULTIPLIER {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
            )
            .message(arcstr::literal!(
                "The reward multiplier cannot be greater than 1_000_000"
            ))
            .param(arcstr::literal!("rewardMultiplier")));
        }

        for milestone in &self.milestones {
            if let Some(error) = milestone.validate() {
                return Err(AppError::new_with_status(
                    warp::http::StatusCode::BAD_REQUEST,
                    INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
                )
                .message(error)
                .param(arcstr::literal!("milestones")));
            }
        }

        Ok(())
    }
}
