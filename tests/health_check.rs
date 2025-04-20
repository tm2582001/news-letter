// `tokio::test` is the testing equivalent of `tokio::main`.
// it also spares you from having to specify the `#[test]` attribute.
//
// you can inspect what code gets generate using
// `cargo expand --test health_check` (<-name of test file)

use std::net::TcpListener;
use news_letter::configuration::{self, get_configuration};
use sqlx::{PgConnection, Connection};

use news_letter::startup::run;




#[tokio::test]
async fn dummy_test() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("failed to bind address");
    // Launch the server as the background tasl
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence no binding

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!

    println!("{}",connection_string);

    let mut connection = PgConnection::connect(&connection_string)
    .await
    .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&mut connection)
    .await.expect("Failed to fetch saved subscription.");
assert_eq!(saved.email, "ursula_le_guin@gmail.com");
assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional constomised error message on test failure
            "The Api not fail with 400 Bad Request when payload was {}.",
            error_message
        )
    }
}
