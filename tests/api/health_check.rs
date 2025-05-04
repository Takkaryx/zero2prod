//! tests/health_check.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let application = spawn_app().await;

    // Act
    let response = application.post_health_check().await;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
