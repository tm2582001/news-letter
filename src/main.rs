use news_letter::configuration::get_configuration;
use news_letter::startup::run;
use news_letter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("news-letter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configurations.");

    //? single connection
    // let connection = PgConnection::connect(
    //     &configuration.database.connection_string()
    // ).await
    // .expect("Failed to connect to Postgres");

    //? connection pool
    let connection_pool =
        PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!("{}:{}",configuration.application.host, configuration.application.port);

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

// #[cfg(test)]
// mod tests {
//     use crate::health_check;
//     #[tokio::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;
//         // This requires changing the return type of `health_check`
//         // from `impl Responder` to `HttpResponse` to compile
//         // You also need to import it with `use actix_web::HttpResponse`!
//         assert!(response.status().is_success())
//     }
// }
