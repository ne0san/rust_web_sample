#[derive(Debug, Clone, PartialEq)]
pub struct UserName(String);
impl UserName {
    fn new(name: &str) -> Result<Self, String> {
        if name.len() < 3 {
            return Err("Username must be at least 3 characters long".to_string());
        }
        if name.len() > 16 {
            return Err("Username must be at most 16 characters long".to_string());
        }
        Ok(UserName(name.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct UserNameFactory {
    user_name_validator: fn(&str) -> bool,
}
impl UserNameFactory {
    pub fn new(user_name_validator: fn(&str) -> bool) -> Self {
        Self {
            user_name_validator,
        }
    }
    pub fn create(&self, name: &str) -> Result<UserName, String> {
        if !(self.user_name_validator)(name) {
            return Err("Username contains NG word".to_string());
        }
        Ok(UserName::new(name)?)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NgWord(String);
impl NgWord {
    pub fn new(word: &str) -> Result<Self, String> {
        if word.len() < 3 {
            return Err("NgWord must be at least 3 characters long".to_string());
        }
        if word.len() > 16 {
            return Err("NgWord must be at most 16 characters long".to_string());
        }
        Ok(NgWord(word.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod user_name {
        use super::*;
        #[test]
        fn test_valid_username() {
            let username = UserName::new("validname");
            assert!(username.is_ok());
            assert_eq!(username.unwrap().as_str(), "validname");
        }
        #[test]
        fn test_username_too_short() {
            let username = UserName::new("ab");
            assert!(username.is_err());
            assert_eq!(
                username.err().unwrap(),
                "Username must be at least 3 characters long"
            );
        }
        #[test]
        fn test_username_too_long() {
            let username = UserName::new("thisisaverylongusername");
            assert!(username.is_err());
            assert_eq!(
                username.err().unwrap(),
                "Username must be at most 16 characters long"
            );
        }
        #[test]
        fn test_username_edge_case_min_length() {
            let username = UserName::new("abc");
            assert!(username.is_ok());
            assert_eq!(username.unwrap().as_str(), "abc");
        }
        #[test]
        fn test_username_edge_case_max_length() {
            let username = UserName::new("sixteencharacter");
            assert!(username.is_ok());
            assert_eq!(username.unwrap().as_str(), "sixteencharacter");
        }
    }
    mod ng_word {
        use super::*;
        #[test]
        fn test_valid_ng_word() {
            let ng_word = NgWord::new("validword");
            assert!(ng_word.is_ok());
            assert_eq!(ng_word.unwrap().as_str(), "validword");
        }
        #[test]
        fn test_ng_word_too_short() {
            let ng_word = NgWord::new("ab");
            assert!(ng_word.is_err());
            assert_eq!(
                ng_word.err().unwrap(),
                "NgWord must be at least 3 characters long"
            );
        }
        #[test]
        fn test_ng_word_too_long() {
            let ng_word = NgWord::new("thisisaverylongword");
            assert!(ng_word.is_err());
            assert_eq!(
                ng_word.err().unwrap(),
                "NgWord must be at most 16 characters long"
            );
        }
        #[test]
        fn test_ng_word_edge_case_min_length() {
            let ng_word = NgWord::new("abc");
            assert!(ng_word.is_ok());
            assert_eq!(ng_word.unwrap().as_str(), "abc");
        }
        #[test]
        fn test_ng_word_edge_case_max_length() {
            let ng_word = NgWord::new("sixteencharacter");
            assert!(ng_word.is_ok());
            assert_eq!(ng_word.unwrap().as_str(), "sixteencharacter");
        }
    }
    mod user_name_factory {
        use super::*;
        #[test]
        fn test_create_valid_username() {
            let factory = UserNameFactory::new(|_| true);
            let username = factory.create("validname");
            assert!(username.is_ok());
            assert_eq!(username.unwrap().as_str(), "validname");
        }
        #[test]
        fn test_create_invalid_username() {
            let factory = UserNameFactory::new(|_| false);
            let username = factory.create("invalidname");
            assert!(username.is_err());
            assert_eq!(username.err().unwrap(), "Username contains NG word");
        }
    }
}
