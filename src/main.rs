use news_letter::configuration::get_configuration;
use news_letter::startup::run;
use std::net::TcpListener;
use sqlx::{Connection, PgPool};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configurations.");

    //? single connection
    // let connection = PgConnection::connect(
    //     &configuration.database.connection_string()
    // ).await
    // .expect("Failed to connect to Postgres");

    //? connection pool
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    ).await
    .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);

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
