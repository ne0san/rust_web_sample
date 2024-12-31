use super::constants::{
    MAX_NG_WORD_LENGTH, MAX_USER_NAME_LENGTH, MIN_NG_WORD_LENGTH, MIN_USER_NAME_LENGTH,
};
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
    /// use domain_model::post::model::UserName;
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
    /// use domain_model::post::model::UserName;
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
    /// use domain_model::post::model::UserName;
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

#[derive(Debug, Clone, PartialEq)]
pub struct NgWord(String);
impl NgWord {
    /// NGワードの値オブジェクトを生成
    ///
    /// NGワードが3文字未満もしくは10文字を超える場合はエラーを返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domain_model::post::model::NgWord;
    ///
    /// let result = NgWord::new("name");
    /// assert!(result.is_ok());
    ///
    /// let result = NgWord::new("name_b");
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// ```rust
    /// use domain_model::post::model::NgWord;
    ///
    /// // 3文字未満の場合
    /// let result = NgWord::new("sh");
    /// assert!(result.is_err());
    ///
    /// // 16文字を超える場合
    /// let result = NgWord::new("thisisaverylo");
    /// assert!(result.is_err());
    /// ```
    pub fn new(ng_word: &str) -> Result<Self, ValidationError> {
        if ng_word.len() < MIN_NG_WORD_LENGTH {
            Err(ValidationError(format!(
                "NgWord must be at least {} characters long",
                MIN_NG_WORD_LENGTH
            )))
        } else if ng_word.len() > MAX_NG_WORD_LENGTH {
            Err(ValidationError(format!(
                "NgWord must be at most {} characters long",
                MAX_NG_WORD_LENGTH
            )))
        } else {
            Ok(NgWord(ng_word.to_string()))
        }
    }

    /// NGワードの値オブジェクトの値を取得
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domain_model::post::model::NgWord;
    ///
    /// let ng_word = NgWord::new("name").unwrap();
    /// assert_eq!(ng_word.value(), "name");
    ///
    /// let ng_word = NgWord::new("name_b").unwrap();
    /// assert_eq!(ng_word.value(), "name_b");
    /// ```
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnvalidatedPost {
    pub user_name: UnvalidatedUserName,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    user_name: UserName,
    content: String,
}
impl Post {
    /// 投稿の値オブジェクトを生成
    ///
    /// ユーザ名が3文字未満もしくは16文字を超える場合はエラーを返す
    /// コンテンツが空文字の場合はエラーを返す
    ///
    /// # Examples
    /// ```rust
    /// use domain_model::post::model::Post;
    ///
    /// let user_name = "name".to_string();
    /// let content = "content".to_string();
    ///
    /// let post = Post::new(user_name, content);
    /// assert!(post.is_ok());
    /// ```
    ///
    /// # Failures
    /// ```rust
    /// use domain_model::post::model::Post;
    ///
    /// // ユーザ名が3文字未満の場合
    /// let user_name = "na".to_string();
    /// let content = "content".to_string();
    /// let post = Post::new(user_name, content);
    /// assert!(post.is_err());
    ///
    /// // ユーザ名が16文字を超える場合
    /// let user_name = "thisisaverylongname".to_string();
    /// let content = "content".to_string();
    /// let post = Post::new(user_name, content);
    /// assert!(post.is_err());
    ///
    /// // コンテンツが空文字の場合
    /// let user_name = "name".to_string();
    /// let content = "".to_string();
    /// let post = Post::new(user_name, content);
    /// assert!(post.is_err());
    /// ```
    pub fn new(user_name: String, content: String) -> Result<Self, ValidationError> {
        let user_name = UserName::new(&user_name)?;
        if content.len() == 0 {
            Err(ValidationError("Content must not be empty".to_string()))
        } else {
            Ok(Self { user_name, content })
        }
    }
    pub fn user_name(&self) -> &UserName {
        &self.user_name
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
