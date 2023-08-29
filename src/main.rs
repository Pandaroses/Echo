mod auth;
mod generation;
mod models;
mod account;
mod schemas;

use actix_web::{ HttpResponse , HttpServer , App , middleware::Logger, web};
use diesel_async::AsyncMysqlConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use actix_cors::Cors;

#[derive(Clone)]
pub struct State {
    pool : Pool<AsyncMysqlConnection>
}
impl State {
    fn connect() -> std::io::Result<Self> {
        let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(std::env::var("DATABASE_URL").unwrap_or("mysql://root@localhost:3306/echo".to_string()));
        let pool = Pool::builder(config).build().expect("Failed to connect wa");
        Ok(Self { pool })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(State::connect().expect("Failed")))
        .wrap(Logger::new("%U %s %Dms"))
        .wrap(Cors::default()
                .allowed_methods(vec!["GET","POST"])
                .supports_credentials()
                .allowed_origin("http://localhost:8080")
        )
        .service(
            web::scope("/auth")
                    .service(auth::register)
                    .service(auth::login)
        )
        .service(web::scope("/account")
                    .service(account::details)
        )
        .service(actix_files::Files::new("/" , "./client/dist").index_file("index.html"))
        .default_service(web::to(|| async {
            HttpResponse::Ok().body(std::fs::read_to_string("./client/dist/index.html").unwrap())
        }))
    }).bind("127.0.0.1:8080")?.run().await?;
    Ok(())
}