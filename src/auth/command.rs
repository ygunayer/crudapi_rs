pub struct InitiateLogin {
    pub email: String,
    pub password: String
}

pub struct RegisterCommand {
    pub first_name: String,
    pub last_name: String,
    pub organization_name: String,
    pub email: String,
    pub gsm_number: String,
    pub password: String,
    pub password_repeat: String,
}
