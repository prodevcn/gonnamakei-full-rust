use std::ops::Deref;

use lazy_static::lazy_static;

use commons::data::games::ClashRoyaleArena;

use crate::models::responses::data::games::clash_royale::arenas::ClashRoyaleArenaGameDataResponse;
use crate::routes::RequestContext;

lazy_static! {
    static ref ARENAS_LIST: Vec<ClashRoyaleArenaGameDataResponse> = ClashRoyaleArena::enum_list()
        .iter()
        .map(|v| { ClashRoyaleArenaGameDataResponse::new(*v) })
        .collect();
}

pub async fn arenas_service(
    _request_context: RequestContext<()>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let list = ARENAS_LIST.deref();
    Ok(warp::reply::json(list))
}
