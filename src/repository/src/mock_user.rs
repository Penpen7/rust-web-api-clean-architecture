pub struct MockUserRepository {}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl application::repository::UserRepository for MockUserRepository {
    fn find_by_mail_address(&self, mail_address: &domain::MailAddress) -> domain::User {
        let name = domain::UserName::new("hogehoge".to_string()).unwrap();
        domain::User::new(name, mail_address.clone())
    }
}
