pub use clash_royale::*;
use commons::database::documents::APIParticipantGamesData;
use commons::database::NullableOption;
use commons::error::AppResult;

mod clash_royale;

pub fn validate_participant_games_data(data: &APIParticipantGamesData) -> AppResult<()> {
    if let NullableOption::Value(clash_royale) = &data.clash_royale {
        validate_participant_clash_royale_games_data(clash_royale)?;
    }

    Ok(())
}
