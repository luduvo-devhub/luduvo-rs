use luduvo_api::prelude::*;
use serde_json::json;
use tokio::time;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn setup_wrapper(server: &MockServer) -> QueryClient {
    let config = QueryConfig::new(
        None,
        Some(format!("{}/users", server.uri())),
        Some(1)
    );

    QueryClient::new(Some(config))
}

fn mock_query_body() -> serde_json::Value {
    json!([
        {
            "id": 1,
            "username": "Luduvo",
            "display_name": "Luduvo",
            "role": "user",
            "head_color": "#FFFFFF",
            "torso_color": "#FFFFFF",
            "created_at": 1234567890
        }
    ])
}

#[tokio::test]
async fn get_query_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("q", "Luduvo"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_query_body()))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);
    let result = wrapper.get_user("Luduvo".to_string(), None).await.unwrap();

    assert_eq!(result.users.len(), 1);
    assert_eq!(result.users[0].id, 1);
    assert_eq!(result.users[0].username, "Luduvo");
}

#[tokio::test]
async fn get_query_rate_limited() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("q", "Luduvo"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(429))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_user("Luduvo".to_string(), None).await {
        Err(QueryError::TooManyRequests()) => {}
        Err(QueryError::InternalError(_)) => {}
        Err(QueryError::RequestFailed(_)) => {}

        other => panic!("expected TooManyRequests, got {:?}", other),
    }
}

#[tokio::test]
async fn get_query_cache_hit() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("q", "Luduvo"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_query_body()))
        .expect(1)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let _ = wrapper.get_user("Luduvo".to_string(), None).await.unwrap();
    let _ = wrapper.get_user("Luduvo".to_string(), None).await.unwrap();
}

#[tokio::test]
async fn get_query_cache_expiration() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("q", "Luduvo"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_query_body()))
        .expect(2)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);
    let _ = wrapper.get_user("Luduvo".to_string(), None).await.unwrap();

    time::sleep(std::time::Duration::from_secs(2)).await;

    let _ = wrapper.get_user("Luduvo".to_string(), None).await.unwrap();
}

#[tokio::test]
async fn get_query_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("q", "Luduvo"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_user("Luduvo".to_string(), None).await {
        Err(QueryError::RequestFailed(_)) => {}
        Err(QueryError::InternalError(_)) => {}

        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

#[tokio::test]
async fn get_query_invalid_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("q", "Luduvo"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_user("Luduvo".to_string(), None).await {
        Err(QueryError::RequestFailed(_)) => {}
        Err(QueryError::InternalError(_)) => {}

        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

#[tokio::test]
async fn query_fields_are_valid() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("q", "Luduvo"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_query_body()))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let result = wrapper.get_user("Luduvo".to_string(), None).await.unwrap();
    let user = &result.users[0];

    assert!(!user.username.is_empty());
    assert!(!user.display_name.is_empty());
    assert!(user.head_color.starts_with('#'));
    assert_eq!(user.head_color.len(), 7);
}
