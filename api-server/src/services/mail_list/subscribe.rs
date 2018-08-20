use commons::database::documents::EmailDBDocument;
use commons::database::{DBDocument, NullableOption};

use crate::models::requests::mail_list::MailListSubscribeRequestBody;
use crate::routes::RequestContext;

pub async fn subscribe_service(
    request_context: RequestContext<MailListSubscribeRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request = request_context.request;

    // Validate input.
    request.validate()?;

    // Persist email.
    let email = EmailDBDocument {
        email: NullableOption::Value(request.email),
        ..Default::default()
    };

    let _ = email.insert_and_ignore(false).await;

    Ok(warp::reply())
}
