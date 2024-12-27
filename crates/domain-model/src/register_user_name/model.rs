use super::constants::{MAX_USER_NAME_LENGTH, MIN_USER_NAME_LENGTH};
use super::err::ValidationError;

#[derive(Debug, Clone, PartialEq)]
pub struct UnvalidatedUserName(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct UserName(String);
impl UserName {
    /// ユーザ名の値オブジェクトを生成
    ///
    /// ユーザ名が3文字未満もしくは16文字を超える場合はエラーを返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domain_model::register_user_name::model::UserName;
    ///
    /// let result = UserName::new("name");
    /// assert!(result.is_ok());
    ///
    /// let result = UserName::new("name_b");
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// ```rust
    /// use domain_model::register_user_name::model::UserName;
    ///
    /// // 3文字未満の場合
    /// let result = UserName::new("sh");
    /// assert!(result.is_err());
    ///
    /// // 16文字を超える場合
    /// let result = UserName::new("thisisaverylongname");
    /// assert!(result.is_err());
    /// ```
    pub fn new(name: &str) -> Result<Self, ValidationError> {
        if name.len() < MIN_USER_NAME_LENGTH {
            Err(ValidationError(format!(
                "Username must be at least {} characters long",
                MIN_USER_NAME_LENGTH
            )))
        } else if name.len() > 16 {
            Err(ValidationError(format!(
                "Username must be at most {} characters long",
                MAX_USER_NAME_LENGTH
            )))
        } else {
            Ok(UserName(name.to_string()))
        }
    }

    /// ユーザ名の値オブジェクトの値を取得
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domain_model::register_user_name::model::UserName;
    ///
    /// let username = UserName::new("name").unwrap();
    /// assert_eq!(username.value(), "name");
    ///
    /// let username = UserName::new("name_b").unwrap();
    /// assert_eq!(username.value(), "name_b");
    /// ```
    pub fn value(&self) -> &str {
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
            assert_eq!(username.unwrap().value(), "validname");
        }
        #[test]
        fn test_username_too_short() {
            let username = UserName::new("ab");
            assert!(username.is_err());
            assert_eq!(
                username.err().unwrap(),
                ValidationError("Username must be at least 3 characters long".to_string())
            );
        }
        #[test]
        fn test_username_too_long() {
            let username = UserName::new("thisisaverylongusername");
            assert!(username.is_err());
            assert_eq!(
                username.err().unwrap(),
                ValidationError("Username must be at most 16 characters long".to_string())
            );
        }
        #[test]
        fn test_username_edge_case_min_length() {
            let username = UserName::new("abc");
            assert!(username.is_ok());
            assert_eq!(username.unwrap().value(), "abc");
        }
        #[test]
        fn test_username_edge_case_max_length() {
            let username = UserName::new("sixteencharacter");
            assert!(username.is_ok());
            assert_eq!(username.unwrap().value(), "sixteencharacter");
        }
    }
}
