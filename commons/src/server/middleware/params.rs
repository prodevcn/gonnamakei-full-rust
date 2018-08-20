use std::str::FromStr;

use warp::Filter;

/// Ads an optional parameter.
pub fn optional_param<T>(
) -> impl Filter<Extract = (Option<T>,), Error = std::convert::Infallible> + Clone + Send + Sync + 'static
where
    T: FromStr + Send + 'static,
{
    warp::any().and(
        warp::path::param::<T>()
            .map(Some)
            .or_else(|_| async { Ok::<(Option<T>,), std::convert::Infallible>((None,)) }),
    )
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Ads an optional parameter with end.
pub fn optional_param_and_end<T>(
) -> impl Filter<Extract = (Option<T>,), Error = warp::Rejection> + Clone + Send + Sync + 'static
where
    T: FromStr + Send + 'static,
{
    optional_param::<T>().and(warp::path::end())
}
