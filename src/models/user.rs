// use std::future::Future;
use std::{future::Future, pin::Pin, sync::Arc};

use serde::Deserialize;

use cookie::Cookie;
use ntex::{
    http::HttpMessage,
    web::{ErrorRenderer, FromRequest},
};
use reqwest::Client;
// use serde::Deserialize;

use crate::{errors::CustomError, AppState};
#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

/// github returned user info
#[derive(Debug, Clone, Deserialize)]
pub struct GithubUserInfo {
    /// github user id
    pub id: i32,
    /// user name not user nick
    pub login: String,
    /// user avatar url
    pub avatar_url: String,
}

/// all usr (include admin) (for id verify)
#[derive(Debug, Clone)]
pub struct User {
    // pub access_token: String,
    pub id: i32,
}

/// web admin (used for id verify)
#[derive(Debug, Clone)]
pub struct Admin {
    pub id: i32,
}
// imple the trait of FromRequest
// extract user info from request, and verify user ID
// 

impl<E: ErrorRenderer> FromRequest<E> for User {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) ->
    Self::Future {
        // attension: below 2 vars, cannot use referrence(req), otherwise, will have issue of life cycle
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap()).db_pool.clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("You have not logon".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return  Err(e);
                }
            };

            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
            .fetch_optional(&db_pool)
            .await
            .unwrap()
            .is_none() {
                // there is no record, cur usr, did not log on our web site, use github
                return Err(CustomError::AuthFailed("You did never log on our web using github".into()));
            }
            Ok(Self {id: user_id})
        };
        
        Box::pin(fut)

    }
    
}

impl <E:ErrorRenderer> FromRequest<E> for Admin {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) -> Self::Future {
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("You still not log on".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return  Err(e);
                }
            };
            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await?
                .is_some()
            {
                // GOT and NEED admin privilege
                if user_id != 2322 {
                    return Err(CustomError::AuthFailed("you are not admin, have no privilege to do so".into(),));
                }
            }else {
                // DID NOT GOT 
                // CUR USR DID NOT LOG EVER
                return Err(CustomError::AuthFailed("YOU DID NEVER LOG".into(),));
            }
            Ok(Self {id : user_id})
        };
        Box::pin(fut)

    }
    
}




async fn get_user_id(access_token: &Cookie<'_>) -> Result<i32, CustomError> {
    let client = Client::new();

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.value())
        // Github 的 API 要求我们设置 UA
        .header("User-Agent", "za-songguo")
        .send()
        .await;

    let user_id = match user_info {
        Ok(r) => {
            match r.json::<GithubUserInfo>().await {
                Ok(i) => i.id,
                Err(_) =>
                // 无法解析，可能是 Github 返回了错误消息
                {
                    return Err(CustomError::BadRequest(
                        "无法获取 Github 用户信息，可能是提供了不正确的 access token，请重新登录"
                            .into(),
                    ))
                }
            }
        }
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取 Github 用户信息，请重试".into(),
            ))
        }
    };

    Ok(user_id)
}