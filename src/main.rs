// To learn more about http we can see this http implementation
//  https://github.com/hyperium/http/blob/c28945c6c6f99379b674a1e961a743c7752f2346/src/header/name.rs#L981   <--implementation
// https://github.com/hyperium/http/blob/c28945c6c6f99379b674a1e961a743c7752f2346/src/header/value.rs#L780   <--test



use news_letter::configuration::get_configuration;
use news_letter::startup::Application;
use news_letter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("news-letter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configurations.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
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
