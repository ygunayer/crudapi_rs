use serde::Deserialize;

use super::command::{InitiateLogin, RegisterCommand};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationRequest {
    first_name: String,
    last_name: String,
    organization_name: String,
    email: String,
    gsm_number: String,
    password: String,
    password_repeat: String,
}

impl Into<InitiateLogin> for LoginRequest {
    fn into(self) -> InitiateLogin {
        InitiateLogin { email: self.email, password: self.password }
    }
}

impl Into<RegisterCommand> for RegistrationRequest {
    fn into(self) -> RegisterCommand {
        RegisterCommand {
            first_name: self.first_name,
            last_name: self.last_name,
            organization_name: self.organization_name,
            email: self.email,
            gsm_number: self.gsm_number,
            password: self.password,
            password_repeat: self.password_repeat,
        }
    }
}
