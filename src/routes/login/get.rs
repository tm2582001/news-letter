use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::{IncomingFlashMessages};
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();

    for m in flash_messages.iter(){
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    } 

    // we can use tera or askama for templating engine
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
        <!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html;" charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>login</title>
</head>
<body>
{error_html}
    <form action="/login" method="post">
        <label>
            Username
            <input type="text" placeholder="Enter Username" name="username" />
        </label>
        <label>
            Password
            <input type="password" placeholder="Enter Password" name="password" />
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>
        "#
        ))
}
