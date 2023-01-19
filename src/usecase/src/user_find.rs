pub trait UserFindUsecase {
    fn handle(&self, req: UserFindUsecaseRequest) -> Result<UserFindUsecaseResponse, String>;
}

pub struct UserFindUsecaseRequest {
    mail_address: domain::MailAddress,
}

impl UserFindUsecaseRequest {
    pub fn new(mail_address: String) -> Result<UserFindUsecaseRequest, String> {
        let mail_address = domain::MailAddress::new(mail_address)?;
        Ok(Self { mail_address })
    }

    pub fn get_mail_address(&self) -> &domain::MailAddress {
        &self.mail_address
    }
}

pub struct UserFindUsecaseResponse {
    mail_address: String,
    user_name: String,
}

impl UserFindUsecaseResponse {
    pub fn new(user: &domain::User) -> UserFindUsecaseResponse {
        Self {
            mail_address: user.get_mail_address().get_value().to_string(),
            user_name: user.get_user_name().get_value().to_string(),
        }
    }

    pub fn get_mail_address(&self) -> &str {
        &self.mail_address
    }

    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
}
