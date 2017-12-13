use conduit::Method;
use chrono::{DateTime, NaiveDateTime};

/// Test timezone handling on the backend
#[test]
fn test_TZ_handling() {
    let now = NaiveDateTime::from_str("2012-02-22T14:53:18+00:00");
    let (_b, app, middle) = ::app();
    // Set TZ here?
    let mut req = ::req(Arc::clone(&app), Method::Get, "/api/v1/crates");
    let mut response = ok_resp!(middle.call(&mut req));
    let json: CrateList = ::json(&mut response);
    assert_eq!(json.crates.len(), 0);
}
