use std::net::TcpListener;

use news_letter::run;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:8080")?;
   run(address)?.await
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
