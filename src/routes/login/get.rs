use actix_web::cookie::Cookie;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let error_html = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };

    // we can use tera or askama for templating engine
    let mut response = HttpResponse::Ok()
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
        ));

    response.add_removal_cookie(&Cookie::new("_flash", "")).unwrap();
    response
}
