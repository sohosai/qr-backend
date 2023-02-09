use crate::error::{UseCaseError, UseCaseResult};
use qr_context::{AuthenticationContext, UserRepository};
use qr_domain::user::User;

#[derive(Debug, Clone)]
pub enum Error {
    NotSignedUp,
}

#[tracing::instrument(skip(ctx))]
pub async fn run<C>(ctx: C) -> UseCaseResult<User, Error>
where
    C: UserRepository + AuthenticationContext,
{
    if let Some(user) = ctx.get_user(ctx.login_user()).await? {
        Ok(user)
    } else {
        return Err(UseCaseError::UseCase(Error::NotSignedUp));
    }
}
