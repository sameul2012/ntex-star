use std::{task::Context, task::Poll};

use ntex::http::body::{Body, ResponseBody};
use ntex::http::{HttpMessage, Method, client};
use ntex::service::Service;
use ntex::util::BoxFuture;

use ntex::web::{WebRequest, WebResponse};
use ntex::Middleware;
use reqwest::{Client, StatusCode};
use sqlx::{Pool, Postgres};

use crate::models::user::GithubUserInfo;

pub struct CheckLogin {
    // db conn pool
    pub db_pool: Pool<Postgres>,
    // only admin can execute
    pub admin: bool,
}

pub struct CheckLoginMiddleware<S> {
    db_pool: Pool<Postgres>,
    admin: bool,
    service: S,
}

impl<S> Middleware<S> for CheckLogin {
    type Service = CheckLoginMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        CheckLoginMiddleware {
            db_pool: self.db_pool.clone(),
            admin: self.admin,
            service,
        }
    }
}

impl<S,E> Service<WebRequest<E>> for CheckLoginMiddleware<S>
where
    S: Service<WebRequest<E>, Response = WebResponse>,
    E: 'static,
{
    type Response = WebResponse;
    type Error = S::Error;
    type Future<'f> = 
        BoxFuture<'f, Result<Self::Response, Self::Error>> where S: 'f, E:'f;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: WebRequest<E>) -> Self::Future<'_> {
        Box::pin(async move {
            let request_method = req.method().to_owned();

            // GET req no verify
            if request_method != Method::GET {
                let db_pool = &self.db_pool;

                // access_token in cookie
                let cookie = req.cookie("ACCESS_TOKEN");

                // response
                let mut res = self.service.call(req).await?;

                let access_token = match cookie {
                    Some(c) => c,
                    None => {
                        res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
                        res = res.map_body(|_,_| {
                            ResponseBody::from(Body::from_slice("You have not logon".as_bytes()))
                        });
                        return  Ok(res);
                    }
                };

                let client = Client::new();

                let user_info = client
                    .get("https://api.github.com/user")
                    .bearer_auth(access_token.value())
                    .header("User-Agent", "freedomdao")
                    .send()
                    .await;
                
                let user_id = match user_info {
                    Ok(r) => match r.json::<GithubUserInfo>().await {
                        Ok(i) => i.id,
                        Err(_)=> {
                            // cannot analyze, maybe github give not formatted info
                            res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
                            res = res.map_body(|_,_| {
                                ResponseBody::from(Body::from_slice("Can not got github user info".as_bytes()))
                            });
                            return Ok(res);
                        }
                    },
                    Err(_) => {
                        // req err
                        res.response_mut().head_mut().status = StatusCode::INTERNAL_SERVER_ERROR;
                        res = res.map_body(|_, _| {
                            ResponseBody::from(Body::from_slice("cannot get user info".as_bytes(),))
                        });
                        return Ok(res);
                    }
                };

                if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                   .fetch_optional(db_pool)
                   .await
                   .unwrap()
                   .is_some()
                {
                    if self.admin {
                        if user_id == 32323 {
                            Ok(res)
                        } else {
                            res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
                            res = res.map_body(|_, _| {
                                ResponseBody::from(Body::from_slice("you are not admin".as_bytes()))
                            });
                            Ok(res)
                        }
                    }else {
                        Ok(res)
                    }
                } else {
                    Ok(res)
                }
            } else {
                // cur usr has not log on, before
                res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
                res = res.map_body(|_,_| {
                    ResponseBody::from(Body::from_slice("You have not log on before, pls log now".as_bytes(),))
                });
                Ok(res)
            }
        } else {
            let res = self.service.call(req).await?;
            Ok(res)
        }
    })
}

 