use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn change_password_form( flash_messages: IncomingFlashMessages)-> Result<HttpResponse, actix_web::Error>{

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(format!(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html;" charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Change password</title>
</head>
<body>
{msg_html}
    <form action="/admin/password" method="post">
        <label>
            Current password
            <input type="password" placeholder="Enter current password" name="current_password" />
        </label>
        <br />
        <label>
            New password
            <input type="password" placeholder="Enter new password" name="new_password" />
        </label>
        <br/>
        <label>
            Confirm new password
            <input type="password" placeholder="Type the new password again" name="new_password_check" />
        </label>
        <br/>
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>
    "#
        
    )))
}