use crate::account::db::AccountRepository;
use crate::common::error::CoreApiError;

use super::command::InitiateLogin;

#[derive(Clone)]
pub struct AuthService {
    account_repository: AccountRepository
}

impl AuthService {
    pub fn new(account_repository: AccountRepository) -> AuthService {
        AuthService { account_repository }
    }

    pub async fn initiate_login(&self, command: InitiateLogin) -> Result<String, CoreApiError> {
        self.account_repository.find_by_email(&command.email).await?
            .map(|_entity| {
                todo!("map account entity into model")
            })
            .ok_or(CoreApiError::not_found("account.not.found"))
    }
}
