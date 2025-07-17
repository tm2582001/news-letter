// To learn more about http we can see this http implementation
//  https://github.com/hyperium/http/blob/c28945c6c6f99379b674a1e961a743c7752f2346/src/header/name.rs#L981   <--implementation
// https://github.com/hyperium/http/blob/c28945c6c6f99379b674a1e961a743c7752f2346/src/header/value.rs#L780   <--test

use std::fmt::{Debug, Display};
use tokio::task::JoinError;

use news_letter::configuration::get_configuration;
use news_letter::startup::Application;
use news_letter::telemetry::{get_subscriber, init_subscriber};
use news_letter::issue_delivery_worker::run_worker_unitil_stopped;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("news-letter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configurations.");

    let application = Application::build(configuration.clone()).await?;
    // need to use tokio::spawn to achieve parallelism otherwise they will be concurrent
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_unitil_stopped(configuration));

    tokio::select! {
        o = application_task =>report_exit("API", o),
        o = worker_task => report_exit("Background worker", o)
    }
    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl Debug + Display>, JoinError>
) {
    match outcome {
        Ok(Ok(())) =>{
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) =>{
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed ", task_name
            )
        }
        Err(e)=>{
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete", task_name
            )
        }
    }
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
