use luduvo_api::prelude::*;
use serde_json::json;
use tokio::time;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn setup_wrapper(server: &MockServer) -> PlacesClient {
    let config = PlacesConfig::new(
        None,
        Some(format!("{}/places", server.uri())),
        Some(1),
    );

    PlacesClient::new(Some(config))
}

fn mock_places_body() -> serde_json::Value {
    json!({
        "places": [
            {
                "id": 1,
                "owner_id": 42,
                "owner_username": "test_user",
                "title": "Test Place",
                "description": "A test place",
                "access": "public",
                "max_players": 10,
                "visit_count": 100,
                "thumbs_up": 5,
                "thumbs_down": 1,
                "active_players": 2,
                "created_at": 1234567890,
                "updated_at": 1234567890,
                "thumbnail_url": ""
            }
        ],
        "total": 1,
        "limit": 20,
        "offset": 0
    })
}

#[tokio::test]
async fn get_places_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/places"))
        .and(query_param("q", "test"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_places_body()))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let result = wrapper.get_places("test".to_string(), None).await.unwrap();

    assert_eq!(result.places.len(), 1);
    assert_eq!(result.places[0].id, 1);
    assert_eq!(result.places[0].owner_username, "test_user");
}

#[tokio::test]
async fn get_places_rate_limited() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/places"))
        .and(query_param("q", "test"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(429))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_places("test".to_string(), None).await {
        Err(PlacesError::TooManyRequests()) => {}
        other => panic!("expected TooManyRequests, got {:?}", other),
    }
}

#[tokio::test]
async fn get_places_cache_hit() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/places"))
        .and(query_param("q", "test"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_places_body()))
        .expect(1)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let _ = wrapper.get_places("test".to_string(), None).await.unwrap();
    let _ = wrapper.get_places("test".to_string(), None).await.unwrap();
}

#[tokio::test]
async fn get_places_cache_expiration() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/places"))
        .and(query_param("q", "test"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_places_body()))
        .expect(2)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);
    let _ = wrapper.get_places("test".to_string(), None).await.unwrap();

    time::sleep(std::time::Duration::from_secs(2)).await;

    let _ = wrapper.get_places("test".to_string(), None).await.unwrap();
}

#[tokio::test]
async fn get_places_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/places"))
        .and(query_param("q", "test"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_places("test".to_string(), None).await {
        Err(PlacesError::InternalError(_)) => {}
        Err(PlacesError::RequestFailed(_)) => {}
        
        other => panic!("expected server error, got {:?}", other),
    }
}

#[tokio::test]
async fn get_places_invalid_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/places"))
        .and(query_param("q", "test"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_places("test".to_string(), None).await {
        Err(PlacesError::RequestFailed(_)) => {}
        
        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

#[tokio::test]
async fn places_fields_are_valid() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/places"))
        .and(query_param("q", "test"))
        .and(query_param("limit", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_places_body()))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let result = wrapper.get_places("test".to_string(), None).await.unwrap();
    let place = &result.places[0];

    assert!(!place.title.is_empty());
    assert!(!place.owner_username.is_empty());
    assert!(place.max_players > 0);
}
