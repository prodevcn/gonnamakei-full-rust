use reqwest::StatusCode;

use commons::database::collections::EmailCollection;
use commons::database::{DBCollection, NullableOption};
use commons::test::assert_http_response_status;

use crate::models::requests::mail_list::MailListSubscribeRequestBody;
use crate::routes::build_routes;
use crate::tests::common::run_db_test_parallel;

#[test]
fn subscribe_ok() {
    run_db_test_parallel(|context, config, _uid_generator| async move {
        let filter = Box::new(build_routes(&context, &config));

        // Test request.
        let original_email = arcstr::literal!("test1@gonnamakeit.app");
        let response = warp::test::request()
            .path("/email/subscribe")
            .method("POST")
            .json(&MailListSubscribeRequestBody {
                email: original_email.clone(),
            })
            .reply(filter.as_ref())
            .await;

        // Check response.
        assert_http_response_status(&response, StatusCode::OK, true);

        // Check database.
        let collection = EmailCollection::instance();
        let emails = collection
            .get_all(None)
            .await
            .expect("Cannot get all emails from DB");

        let mut found = false;
        for email in emails {
            if let NullableOption::Value(email) = email.email {
                found |= email.as_str() == original_email.as_str();
            }
        }

        assert!(found, "The email was not found");
    });
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[test]
fn subscribe_twice_ok() {
    run_db_test_parallel(|context, config, _uid_generator| async move {
        let filter = Box::new(build_routes(&context, &config));

        // Test request.
        let original_email = arcstr::literal!("test2@gonnamakeit.app");
        let response = warp::test::request()
            .path("/email/subscribe")
            .method("POST")
            .json(&MailListSubscribeRequestBody {
                email: original_email.clone(),
            })
            .reply(filter.as_ref())
            .await;

        // Check response.
        assert_http_response_status(&response, StatusCode::OK, true);

        // Test request.
        let response = warp::test::request()
            .path("/email/subscribe")
            .method("POST")
            .json(&MailListSubscribeRequestBody {
                email: original_email.clone(),
            })
            .reply(filter.as_ref())
            .await;

        // Check response.
        assert_http_response_status(&response, StatusCode::OK, true);

        // Check database.
        let collection = EmailCollection::instance();
        let emails = collection
            .get_all(None)
            .await
            .expect("Cannot get all emails from DB");

        let mut found = 0;
        for email in emails {
            if let NullableOption::Value(email) = email.email {
                if email.as_str() == original_email.as_str() {
                    found += 1;
                }
            }
        }

        assert_eq!(found, 1, "The email must be present just one time");
    });
}
