use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::middleware::Logger;

use env_logger::Env;

use crate::routes::{health_check, subscribe};


pub fn run(listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {


    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move|| {
        App::new()
            // Middlewares are added using the `wrap` methon on `App`
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
