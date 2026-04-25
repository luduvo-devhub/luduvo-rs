use luduvo_api::prelude::*;
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

fn setup_wrapper(server: &MockServer) -> FriendsClient {
    let config = FriendsConfig::new(
        None,
        Some(format!("{}/users", server.uri())),
        Some(1)
    );
    
    FriendsClient::new(Some(config))
}

fn mock_profile_body() -> serde_json::Value {
    json!({
        "friends": [],
        "total": 0,
        "limit": 50,
        "offset": 0
    })
}

#[tokio::test]
async fn get_friends_success() {
    let server = MockServer::start().await;
    let body = mock_profile_body();

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);
    let friends = wrapper.get_friends("1".to_string()).await.unwrap();

    assert_eq!(friends.total, 0);
    assert_eq!(friends.friends.len(), 0);
}

#[tokio::test]
async fn get_friends_invalid_id() {
    let mut wrapper = FriendsClient::new(None);

    match wrapper.get_friends("abc".to_string()).await {
        Err(FriendsError::InvalidId(id)) => assert_eq!(id, "abc"),
        Err(FriendsError::InternalError(_)) => {}
        Err(FriendsError::RequestFailed(_)) => {}

        other => panic!("expected InvalidId, got {:?}", other),
    }
}

#[tokio::test]
async fn get_friends_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/999/friends"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_friends("999".to_string()).await {
        Err(FriendsError::ResultNotFound(id)) => assert_eq!(id, "999"),
        Err(FriendsError::InternalError(_)) => {}
        Err(FriendsError::RequestFailed(_)) => {}

        other => panic!("expected ResultNotFound, got {:?}", other),
    }
}

#[tokio::test]
async fn get_friends_cache_hit() {
    let server = MockServer::start().await;
    let body = mock_profile_body();

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let first = wrapper.get_friends("1".to_string()).await.unwrap();
    let second = wrapper.get_friends("1".to_string()).await.unwrap();

    assert_eq!(first.total, second.total);
}

#[tokio::test]
async fn friends_pagination_sanity() {
    let server = MockServer::start().await;
    let body = mock_profile_body();

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let friends = wrapper.get_friends("1".to_string()).await.unwrap();

    assert!(friends.limit > 0);
    assert!(friends.offset <= friends.total);
}

#[tokio::test]
async fn get_friends_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_friends("1".to_string()).await {
        Err(FriendsError::RequestFailed(_)) => {}
        Err(FriendsError::InternalError(_)) => {}

        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

#[tokio::test]
async fn get_friends_invalid_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid"))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_friends("1".to_string()).await {
        Err(FriendsError::RequestFailed(_)) => {}
        Err(FriendsError::InternalError(_)) => {}

        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

#[tokio::test]
async fn get_friends_cache_expiration() {
    let server = MockServer::start().await;
    let body = mock_profile_body();

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(2)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);
    let _ = wrapper.get_friends("1".to_string()).await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let _ = wrapper.get_friends("1".to_string()).await.unwrap();
}
