use std::sync::Arc;

use commons::database::documents::APIParticipantGamesData;
use commons::database::documents::{ParticipantAPIDocument, ParticipantDBDocument};
use commons::database::types::{DBUuid, DBUuidType};
use commons::database::{DBDocument, NullableOption};
use commons::error::AppResult;

use crate::context::AppContext;
use crate::models::requests::participant::ParticipantUpdateRequestBody;
use crate::routes::RequestContextWithAuth;

pub async fn update_service(
    request_context: RequestContextWithAuth<ParticipantUpdateRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let context = request_context.app_context;
    let request = request_context.request;

    // Validate input.
    request.validate()?;
    validate_information(context, &request.games_data).await?;

    // Update participant data.
    let participant_address = request_context.claims.address;
    let participant_key = DBUuid::new_with_content(
        participant_address.as_string().as_str(),
        DBUuidType::Address,
    )
    .unwrap();

    let participant = ParticipantDBDocument {
        db_key: Some(participant_key.clone()),
        games_data: NullableOption::Value(request.games_data.into()),
        ..Default::default()
    };

    let participant = participant.insert_or_update(true).await?;

    let mut response: ParticipantAPIDocument = participant.into();
    response.remove_sensible_info();

    Ok(warp::reply::json(&response))
}

async fn validate_information(
    context: Arc<AppContext>,
    data: &APIParticipantGamesData,
) -> AppResult<()> {
    if let NullableOption::Value(v) = &data.clash_royale {
        if let NullableOption::Value(v) = &v.tag {
            let client = context.game_clients.clash_royale.clone();
            client.get_player_info(v.as_str()).await?;
        }
    }

    Ok(())
}
