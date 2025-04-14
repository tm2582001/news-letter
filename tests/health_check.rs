// `tokio::test` is the testing equivalent of `tokio::main`.
// it also spares you from having to specify the `#[test]` attribute.
// 
// you can inspect what code gets generate using
// `cargo expand --test health_check` (<-name of test file)

#[tokio::test]
async fn dummy_test(){
    // Arrange
    spawn_app();
    
    let client = reqwest::Client::new();

    let response = client.get("http://127.0.0.1:8080/health_check")
            .send()
            .await
            .expect("failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
}

fn spawn_app(){
    let server = news_letter::run().expect("failed to bind address");
    // Launch the server as the background tasl
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence no binding

    let _ = tokio::spawn(server);
}