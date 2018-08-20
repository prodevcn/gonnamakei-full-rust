use std::ops::Deref;

use lazy_static::lazy_static;

use commons::data::games::ClashRoyaleCard;

use crate::models::responses::data::games::clash_royale::cards::ClashRoyaleCardGameDataResponse;
use crate::routes::RequestContext;

lazy_static! {
    static ref CARDS_LIST: Vec<ClashRoyaleCardGameDataResponse> = ClashRoyaleCard::enum_list()
        .iter()
        .map(|v| { ClashRoyaleCardGameDataResponse::new(*v) })
        .collect();
}

pub async fn cards_service(
    _request_context: RequestContext<()>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let list = CARDS_LIST.deref();
    Ok(warp::reply::json(list))
}
