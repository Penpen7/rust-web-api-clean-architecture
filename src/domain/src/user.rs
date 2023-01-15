use regex::Regex;

pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: String) -> Result<UserName, String> {
        if value.chars().count() > 20 {
            return Err("user name is too long".to_string());
        }

        Ok(UserName { value })
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

pub struct MailAddress {
    value: String,
}

impl MailAddress {
    pub fn new(value: String) -> Result<MailAddress, String> {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if email_regex.is_match(&value) {
            Ok(MailAddress { value })
        } else {
            Err("Email Address is not a valid".to_string())
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

pub struct User {
    user_name: UserName,
    mail_address: MailAddress,
}

impl User {
    pub fn new(user_name: UserName, mail_address: MailAddress) -> User {
        User {
            user_name,
            mail_address,
        }
    }

    pub fn get_user_name(&self) -> &UserName {
        &self.user_name
    }

    pub fn get_mail_address(&self) -> &MailAddress {
        &self.mail_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_name_test() {
        let user_names = ["hello", "penpen7", "Penpen7", "ペンペン7"];
        for user_name in user_names {
            assert_eq!(
                UserName::new(user_name.to_string()).unwrap().get_value(),
                user_name
            );
        }
    }

    #[test]
    fn user_name_err_test() {
        let user_names = ["あああああああああああああああああああああ"];
        for user_name in user_names {
            assert!(UserName::new(user_name.to_string()).is_err())
        }
    }

    #[test]
    fn mail_address_test() {
        let mail_addresses = [
            "hogehoge@example.com",
            "fugafuga@example.com",
            "fuga.fuga@example.com",
            "fuga.fuga@example.jp",
        ];
        for mail_address in mail_addresses {
            assert_eq!(
                MailAddress::new(mail_address.to_string())
                    .unwrap()
                    .get_value(),
                mail_address
            );
        }
    }

    #[test]
    fn mail_address_err_test() {
        let mail_addresses = [
            "fuga  fuga@example.com",
            "fuga@fuga@example.com",
            "fuga.fuga.@example.com",
        ];
        for mail_address in mail_addresses {
            assert!(
                MailAddress::new(mail_address.to_string()).is_err(),
                "mail_address {}",
                mail_address
            );
        }
    }

    #[test]
    fn user_test() {
        let user = User::new(
            UserName::new("test".to_string()).unwrap(),
            MailAddress::new("hoge@example.com".to_string()).unwrap(),
        );
        assert_eq!(user.get_user_name().get_value(), "test");
        assert_eq!(user.get_mail_address().get_value(), "hoge@example.com");
    }
}
