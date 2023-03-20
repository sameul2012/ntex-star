use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Login{
    pub code: String,
}

#[derive(Debug,Clone,Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

/// github returned user info
#[derive(Debug, Clone, Deserialize)]
pub struct GithubUserInfo{
    /// github user id
    pub id: i32,
    /// user name not user nick
    pub login: String,
    /// user avatar url
    pub avatar_url: String,
}
 