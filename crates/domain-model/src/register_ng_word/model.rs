use super::constants::{MAX_NG_WORD_LENGTH, MIN_NG_WORD_LENGTH};
use super::err::ValidationError;

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
