use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct PostedUserName(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct PostedDatetime(pub NaiveDateTime);

#[derive(Debug, Clone, PartialEq)]
pub struct Content(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    pub posted_user_name: PostedUserName,
    pub posted_datetime: PostedDatetime,
    pub content: Content,
}
