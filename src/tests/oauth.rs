use hyper::{Body, Request};
use thruster::testing;
use tokio::runtime::Runtime;

use crate::app;

#[test]
fn it_should_have_a_ping_route() {
    let _ = Runtime::new().unwrap().block_on(async {
        let app = app::init().await;

        let response = testing::request(
            &app,
            Request::builder()
                .method("GET")
                .uri("/users/github/oauth")
                .body(Body::from(""))
                .unwrap(),
        )
        .await;

        assert!(response.status == 200);
    });
}
