use crate::authentication::UserId;
use crate::utils::{e500, see_other};
use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::idempotency::{save_response, try_processing, IdempotencyKey, NextAction};
use crate::utils::e400;

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    html_content: String,
    text_content: String,
    idempotency_key: String,
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip_all,
    fields(user_id=%*user_id)
)]

pub async fn publish_newsletter(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    // We must destructure the form to avoid upsetting the borrow-checker
    let FormData {
        title,
        text_content,
        html_content,
        idempotency_key,
    } = form.0;

    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;

    let mut transaction = match try_processing(&pool, &idempotency_key, *user_id)
        .await
        .map_err(e500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => {
            success_message().send();
            return Ok(saved_response);
        }
    };

    let issue_id = insert_newsletter_issue(&mut transaction, &title, &text_content, &html_content)
        .await
        .context("Failed to store newsletter issue details")
        .map_err(e500)?;

    enqueue_delivery_tasks(&mut transaction, issue_id)
        .await
        .context("Failed to enqueue delivery tasks")
        .map_err(e500)?;

    // let subscribers = get_confirmed_subscribers(&pool).await.map_err(e500)?;
    // for subscriber in subscribers {
    //     match subscriber {
    //         Ok(subscriber) => {
    //             email_client
    //                 .send_email(
    //                     &subscriber.email,
    //                     &title,
    //                     &html_content,
    //                     &text_content,
    //                 )
    //                 .await
    //                 .with_context(|| {
    //                     format!("Falied to send newsletter issue to {}", subscriber.email)
    //                 })
    //                 .map_err(e500)?;
    //         }
    //         Err(error) => {
    //             tracing::warn!(
    //                 // We record the error chain as structured field on the log record.
    //                 error.cause_chain = ?error,
    //                                     error.message = %error,
    //                 "Skipping a confirmed subscriber \
    //                 Thier stored contact details are invalid"
    //             )
    //         }
    //     }
    // }
    let response = see_other("/admin/newsletters");
    let response = save_response(transaction, &idempotency_key, *user_id, response)
        .await
        .map_err(e500)?;
    success_message().send();
    Ok(response)
}

fn success_message() -> FlashMessage {
    FlashMessage::info("The newsletter issue has been accepted emals will go out shortly!")
}

#[tracing::instrument(skip_all)]
async fn insert_newsletter_issue(
    transaction: &mut Transaction<'_, Postgres>,
    title: &str,
    text_content: &str,
    html_content: &str,
) -> Result<Uuid, sqlx::Error> {
    let newsletter_issue_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO newsletter_issues(
            newsletter_issue_id,
            title,
            text_content,
            html_content,
            published_at
        )
        VALUES ($1, $2, $3, $4, now())
        "#,
        newsletter_issue_id,
        title,
        text_content,
        html_content
    );
    transaction.execute(query).await?;
    Ok(newsletter_issue_id)
}

#[tracing::instrument(skip_all)]
async fn enqueue_delivery_tasks(
    transaction: &mut Transaction<'_, Postgres>,
    newsletter_issue_id: Uuid,
) -> Result<(), sqlx::Error> {

    let query = sqlx::query!(
        r#"
    INSERT INTO issue_delivery_queue(
        newsletter_issue_id,
        subscriber_email
    )
    SELECT $1, email
    FROM subscriptions
    WHERE status = 'confirmed'
    "#,
        newsletter_issue_id
    );
    transaction.execute(query).await?;
    Ok(())
}
