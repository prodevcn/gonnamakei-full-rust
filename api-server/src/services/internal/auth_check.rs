use commons::database::collections::AuthenticationCollection;
use commons::database::types::DBUuid;

use crate::routes::RequestContextWithAuth;

pub async fn auth_check_service(
    request_context: RequestContextWithAuth<DBUuid>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = request_context.request;

    let authorization = AuthenticationCollection::get_by_key_or_reject(&token, None)
        .await
        .is_ok();
    Ok(warp::reply::json(&authorization))
}
