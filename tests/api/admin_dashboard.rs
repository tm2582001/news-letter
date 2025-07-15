use crate::helpers::{assert_is_redirected_to, spawn_app};

#[tokio::test]
async fn logout_clears_session_state(){
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirected_to(&response, "/admin/dashboard");

    let html_page = app.get_admin_dhasboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    let response = app.post_logout().await;
    assert_is_redirected_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    let response = app.get_admin_dhasboard().await;
    assert_is_redirected_to(&response, "/login");
}