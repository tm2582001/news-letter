// `tokio::test` is the testing equivalent of `tokio::main`.
// it also spares you from having to specify the `#[test]` attribute.
// 
// you can inspect what code gets generate using
// `cargo expand --test health_check` (<-name of test file)

use std::net::TcpListener;

#[tokio::test]
async fn dummy_test(){
    // Arrange
    let address = spawn_app();
    
    let client = reqwest::Client::new();

    let response = client.get(&format!("{}/health_check", &address))
            .send()
            .await
            .expect("failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
}

fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = news_letter::run(listener).expect("failed to bind address");
    // Launch the server as the background tasl
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence no binding

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}