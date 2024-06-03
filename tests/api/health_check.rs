use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;

    let response = app
        .api
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
