use hyper::{Body, Request};
use thruster::testing;
use tokio::runtime::Runtime;

use crate::app;

#[test]
fn it_should_have_an_oauth_route() {
    let _ = Runtime::new().unwrap().block_on(async {
        let app = app::init().await;

        let response = testing::request(
            &app,
            Request::builder()
                .method("GET")
                .uri("/users/github/oauth?state=0&code=0")
                .body(Body::from(""))
                .unwrap(),
        )
        .await;

        assert!(response.status == 200);
    });
}

#[test]
fn it_should_require_state_in_oauth() {
    let _ = Runtime::new().unwrap().block_on(async {
        let app = app::init().await;

        let response = testing::request(
            &app,
            Request::builder()
                .method("GET")
                .uri("/users/github/oauth?code=0")
                .body(Body::from(""))
                .unwrap(),
        )
        .await;

        assert!(response.status == 400);
    });
}

#[test]
fn it_should_require_code_in_oauth() {
    let _ = Runtime::new().unwrap().block_on(async {
        let app = app::init().await;

        let response = testing::request(
            &app,
            Request::builder()
                .method("GET")
                .uri("/users/github/oauth?state=0")
                .body(Body::from(""))
                .unwrap(),
        )
        .await;

        assert!(response.status == 400);
    });
}
