use cookie::{time::Duration, Cookie};
use ntex::{
    http::Response,
    web::{
        types::{Json, State},
        Responder, 
    },
};

use reqwest::Client;
use std::sync::Arc;

use crate::{
    errors::CustomError,
    models::user::{AccessToken, GithubUserInfo, Login},
    AppState,
};

const CLIENT_ID: &str = "xxx";
const CLIENT_SECRET: &str = "xxx";

/// get code transferred from github, navigator will operate automatically, no need user any action
pub async fn github_login(
    code: Json<Login>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, CustomError> {
    let code = &code.code;

    // HTTP client
    let client = Client::new();

    // get access_token
    // set Accept as json, github.com api will return JSON data to us
    let access_token = client
        .post(format!(
            "https://github.com/login/oauth/access_token?client_id={CLIENT_ID}&
        client_secret={CLIENT_SECRET}&code={code}"
        ))
        .header("Accept", "application/json")
        .send()
        .await;

    let access_token = match access_token {
        Ok(r) => match r.json::<AccessToken>().await {
            Ok(r) => r.access_token,
            Err(_) => {
                return Err(CustomError::AuthFailed(
                    "code is invalid, pls login thru github".into(),
                ))
            }
        },
        Err(_) => {
            return Err(CustomError::InternalServerErrror(
                "can not get access_token, pls try again".into(),
            ));
        }
    };

    // user-agent should be project name or user name
    // github api rquired us to setup UA
    
    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        
        .header("User-Agent", "freedomdao")
        .send()
        .await;

    let user_info = match user_info{
        Ok(r) => r.json::<GithubUserInfo>().await.unwrap(),
        Err(_) => {
            return Err(CustomError::InternalServerErrror(
                "can not got github user info".into(),
            ));
        }
    };

    // set cookie, so avoid login again
    // path must be set, so all these path in this web site cookie available
    
    let mut cookie = Cookie::new("ACCESS_TOKEN", access_token);
    cookie.set_path("/"); 
    cookie.set_max_age(Duration::days(7));
    cookie.set_http_only(true);

    // store user info into postgreSQL
    let db_pool = &state.db_pool;

    // if record existed, update, otherwise insert

    sqlx::query!(
        "INSERT INTO users (id, name, avatar_url) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, avatar_url = $3", 
        user_info.id,
        user_info.login,
        user_info.avatar_url
    )
    .execute(db_pool)
    .await?;

    let mut response = Response::Ok().body(format!("Hi, {}",user_info.login)); 

    // ignore error
    let _ = response.add_cookie(&cookie);

    Ok(response)

}
