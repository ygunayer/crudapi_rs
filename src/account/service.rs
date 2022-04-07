use crate::AccountRepository;
use crate::auth::command::RegisterCommand;
use crate::common::error::CoreApiError;

#[derive(Clone)]
pub struct AccountService {
    account_repository: AccountRepository
}

impl AccountService {
    pub fn new(account_repository: AccountRepository) -> AccountService {
        AccountService { account_repository }
    }

    pub async fn register(&self, command: RegisterCommand) -> Result<String, CoreApiError> {
        self.validate_email_is_unique(command.email).await?;
        todo!("insert account and return model")
    }

    async fn validate_email_is_unique(&self, email: String) -> Result<(), CoreApiError> {
        let exists = &self.account_repository.exists_by_email(&email).await?;
        if *exists {
            Err(CoreApiError::business_validation("email.already.exists")) // TODO
        } else {
            Ok(())
        }
    }
}
