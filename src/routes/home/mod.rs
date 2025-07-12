use actix_web::{http::header::ContentType, HttpResponse};

pub async fn home() -> HttpResponse {
    // Read this to know more about 'static https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#common-rust-lifetime-misconceptions
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("home.html"))
}
