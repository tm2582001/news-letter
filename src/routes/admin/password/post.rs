use actix_web::{web, HttpResponse};
use secrecy::{ExposeSecret, SecretBox};
use actix_web_flash_messages::FlashMessage;
use sqlx::PgPool;

use crate::utils::{e500, see_other};
use crate::routes::admin::dashboard::get_username;
use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::authentication::UserId;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: SecretBox<str>,
    new_password: SecretBox<str>,
    new_password_check: SecretBox<str>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error("you entered two different new passwords - \
        the field values must match.").send();
        return Ok(see_other("/admin/password"));
    }

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    let credentials = Credentials {
        username,
        password: form.0.current_password
    };

    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e).into())
        }
    }
    
    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;

    FlashMessage::error("Your password has been changed.").send();
    Ok(see_other("/admin/password"))
}