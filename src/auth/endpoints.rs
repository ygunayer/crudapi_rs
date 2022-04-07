use actix_web::{post, Responder, Result, web};
use actix_web::web::ServiceConfig;

use crate::AuthService;
use crate::account::service::AccountService;
use crate::auth::command::{InitiateLogin, RegisterCommand};
use crate::auth::dto::RegistrationRequest;
use crate::common::response::response_of;

use super::dto::LoginRequest;

pub fn configure(config: &mut ServiceConfig) {
    config.service(login).service(register);
}

#[post("/login")]
async fn login(req: web::Json<LoginRequest>, auth_service: web::Data<AuthService>) -> Result<impl Responder> {
    let cmd: InitiateLogin = req.into_inner().into();
    auth_service.initiate_login(cmd).await
        .map(|result| response_of(result))
        .map_err(|err| err.into())
}

#[post("/register")]
async fn register(req: web::Json<RegistrationRequest>, account_service: web::Data<AccountService>) -> Result<impl Responder> {
    let cmd: RegisterCommand = req.into_inner().into();
    account_service.register(cmd).await
        .map(|result| response_of(result))
        .map_err(|err| err.into())
}
