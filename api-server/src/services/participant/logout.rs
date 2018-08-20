use commons::database::collections::AuthenticationCollection;

use crate::routes::RequestContextWithAuth;

pub async fn logout_service(
    request_context: RequestContextWithAuth<()>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let address = request_context.claims.address;

    // Remove a previous authorization.
    AuthenticationCollection::remove_by_address_or_reject(&address).await?;

    Ok(warp::reply())
}
