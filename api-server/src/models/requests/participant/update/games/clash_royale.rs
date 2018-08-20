use commons::data::games::clash_royale_tag_validator;
use commons::database::documents::APIParticipantClashRoyaleGameData;
use commons::database::NullableOption;
use commons::error::{AppError, AppResult, INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE};

pub fn validate_participant_clash_royale_games_data(
    data: &APIParticipantClashRoyaleGameData,
) -> AppResult<()> {
    if let NullableOption::Value(tag) = &data.tag {
        if !clash_royale_tag_validator(tag.as_str()) {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
            )
            .message(arcstr::literal!("The Clash Royale tag is incorrect"))
            .param(arcstr::literal!("clashRoyaleUserTag")));
        }
    }

    Ok(())
}
