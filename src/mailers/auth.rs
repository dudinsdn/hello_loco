// auth mailer
#![allow(non_upper_case_globals)]

use loco_rs::prelude::*;
use serde_json::json;

use crate::models::users;

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static forgot: Dir<'_> = include_dir!("src/mailers/auth/forgot");
static magic_link: Dir<'_> = include_dir!("src/mailers/auth/magic_link");

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    /// Sending welcome email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &welcome,
            mailer::Args {
                to: user.email.clone(),
                locals: json!({
                  "name": user.name,
                  "verifyToken": user.email_verification_token,
                  "host": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sending forgot password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &forgot,
            mailer::Args {
                to: user.email.clone(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "host": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sends a magic link authentication email to the user.
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_magic_link(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &magic_link,
            mailer::Args {
                to: user.email.clone(),
                locals: json!({
                  "name": user.name,
                  "token": user.magic_link_token.clone().ok_or_else(|| Error::string(
                            "the user model not contains magic link token",
                    ))?,
                  "host": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}

/// A sample rendering of the same source templates used for outgoing mail.
#[derive(serde::Serialize)]
pub struct EmailPreview {
    pub subject: String,
    pub html: String,
    pub text: String,
    pub subject_source: &'static str,
    pub html_source: &'static str,
    pub text_source: &'static str,
}

impl AuthMailer {
    pub fn preview(kind: &str) -> Result<Option<EmailPreview>> {
        let (subject_source, html_source, text_source) = match kind {
            "welcome" => (
                include_str!("auth/welcome/subject.t"),
                include_str!("auth/welcome/html.t"),
                include_str!("auth/welcome/text.t"),
            ),
            "forgot" => (
                include_str!("auth/forgot/subject.t"),
                include_str!("auth/forgot/html.t"),
                include_str!("auth/forgot/text.t"),
            ),
            "magic_link" => (
                include_str!("auth/magic_link/subject.t"),
                include_str!("auth/magic_link/html.t"),
                include_str!("auth/magic_link/text.t"),
            ),
            _ => return Ok(None),
        };
        let locals = json!({
            "name": "Rani",
            "host": "https://example.invalid",
            "verifyToken": "contoh-verifikasi",
            "resetToken": "contoh-reset",
            "token": "contoh-magic-link",
        });
        Ok(Some(EmailPreview {
            subject: loco_rs::controller::views::template(subject_source, locals.clone())?,
            html: loco_rs::controller::views::template(html_source, locals.clone())?,
            text: loco_rs::controller::views::template(text_source, locals)?,
            subject_source,
            html_source,
            text_source,
        }))
    }
}
