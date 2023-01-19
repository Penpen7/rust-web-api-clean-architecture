pub trait UserRepository {
    fn find_by_mail_address(&self, mail_address: &domain::MailAddress) -> domain::User;
}
