use domain::{MailAddress, User};
pub struct RDBUserRepository {}

impl RDBUserRepository {
    fn new() -> RDBUserRepository {
        RDBUserRepository {}
    }
    fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<User, String> {
        todo!()
    }
}
