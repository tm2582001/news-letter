use crate::helpers::spawn_app;
use crate::helpers::assert_is_redirected_to;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure(){
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });

    let response = app.post_login(&login_body).await;

    // let cookies: HashSet<_> = response
    //     .headers()
    //     .get_all("Set-Cookie")
    //     .into_iter()
    //     .collect();

    // assert!(cookies.contains(&HeaderValue::from_str("_flash=Authentication failed").unwrap()));

    // let flash_cookie = response.cookies().find(|c| c.name() == "_flash").unwrap();
    // assert_eq!(flash_cookie.value(), "Authentication failed");

    assert_is_redirected_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    let html_page = app.get_login_html().await;
    assert!(!html_page.contains("Authentication failed"));


}


#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success(){
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirected_to(&response, "/admin/dashboard");

    let html_page = app.get_admin_dhasboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard(){
    let app = spawn_app().await;

    let response = app.get_admin_dhasboard().await;

    assert_is_redirected_to(&response, "/login");
}