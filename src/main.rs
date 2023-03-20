mod article;
mod errors;
mod middlewars;
mod models;
mod user;

use article::{delete, edit, new, search, view};
use user::login;

use ntex::web::{self, middleware, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use std::{env, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[ntex::main]
async fn main() {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Pls set `DATABASE_URL` in env or env var");

    // State
    let app_state = Arc::new(AppState {
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .configure(|cfg| route(Arc::clone(&app_state), cfg))
        //   .configure(route)
        //   .service(index)
        //   .service(error)
        //    .service(view::get_all_articles)
        //    .service(new::new_article)
        //    .service(edit::edit_article)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap();
}

fn route(_state: Arc<AppState>, cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/article")
            .route("/{id}", web::get().to(view::get_article))
            .route("", web::post().to(new::new_article))
            .route("", web::put().to(edit::edit_article))
            .route("/{id}", web::delete().to(delete::delete_article))
            .route("/search/{keyword}", web::get().to(search::search_article)),
    )
    .service(web::scope("/articles").route("", web::get().to(view::get_articles_preview)))
    .service(web::scope("/user").route("/login", web::post().to(login::github_login)));
}

// #[web::get("/")]
// async fn index()-> String{
//     "Hello, world".into()
// }

// #[web::get("/error")]
// async fn error() -> Result<String, CustomError>{
//     Err(CustomError::NotFound("Not found".into()))
// }
