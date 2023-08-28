mod auth;
mod generation;

use actix_web::{get , Responder , HttpResponse , HttpServer , App , middleware::Logger, web::{self, Redirect}, HttpRequest, http::StatusCode, cookie::Cookie, dev::{ResourcePath, ServiceRequest, ServiceResponse}, HttpResponseBuilder};
use actix_files as fs;
use fs::{FilesService, NamedFile};
use generation::{QuestionTemplate, Difficulty , Question};

pub struct State {
    message : String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let question : Question = QuestionTemplate::generate(Difficulty::Easy);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::new("%U %s %Dms"))
        .service(real)
        .service(
            web::scope("/auth")
                    .service(auth::register)
                    .service(auth::login)
        )
    }).bind(("127.0.0.1" , 8080))?.run().await?;
    Ok(())
}

#[get("/")]
async fn real() -> std::io::Result<impl Responder> {
    Ok(HttpResponse::Ok().body(std::fs::read_to_string("src/main.rs")?))
}

/*.service(fs::Files::new("/" , "./client/dist").index_file("index.html").default_handler(|req : ServiceRequest| async {
            let (req , _) = req.into_parts();
            let file = NamedFile::open_async("./client/dist/index.html").await.unwrap();
            let res = file.into_response(&req);
            Ok(ServiceResponse::new(req , res))
        }))*/