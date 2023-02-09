use crate::error::{UseCaseError, UseCaseResult};

use anyhow::Context as _;
use qr_context::{AuthenticationContext, TimeContext, UserRepository};
use qr_domain::{
    role::Role,
    user::{User, UserName},
};

#[derive(Debug, Clone)]
pub enum Error {
    AlreadySignedUp,
}

#[tracing::instrument(skip(ctx))]
pub async fn run<C>(ctx: C, name: UserName) -> UseCaseResult<User, Error>
where
    C: UserRepository + AuthenticationContext + TimeContext,
{
    let id = ctx.login_user();
    if ctx.get_user(id.clone()).await?.is_some() {
        return Err(UseCaseError::UseCase(Error::AlreadySignedUp));
    }

    let user = User {
        id,
        name,
        email: ctx.login_email(),
        created_at: ctx.now(),
        role: Role::General,
    };
    ctx.create_user(user.clone())
        .await
        .context("Failed to create a user")?;
    Ok(user)
}
