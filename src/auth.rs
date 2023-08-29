use actix_web::{HttpResponseBuilder, HttpRequest};
use actix_web::cookie::{Cookie, CookieBuilder, SameSite};
use actix_web::{post , Responder, HttpResponse , web, http::StatusCode};
use diesel::result::Error::NotFound;
use serde::Deserialize;
use diesel::ExpressionMethods;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use bcrypt::verify;

use crate::models::Session;
use crate::schemas::sessions;
use crate::schemas::users;
use crate::models::User;

#[derive(Deserialize)]
pub struct LoginForm {
    email : String,
    password : String
}

#[derive(Deserialize)]
pub struct RegisterForm {
    email : String,
    password : String,
    first_name : String,
    last_name : String
}

#[post("/register")]
pub async fn register( form : web::Json<RegisterForm> ,state : web::Data<crate::State> ) -> std::io::Result<impl Responder>{
    let mut conn = state.pool.get().await.unwrap();
    let user : User = User::new(form.email.clone() , form.password.clone() , form.first_name.clone() , form.last_name.clone())?;
    Ok(match users::table.filter(users::email.eq(&form.email)).count().get_result::<i64>(&mut conn).await {
        Ok(count) if count < 1 => {
            match diesel::insert_into(users::table)
                .values(user)
                .execute(&mut conn).await{
                Ok(_) => HttpResponse::new(StatusCode::CREATED),
                Err(e) => {
                    println!("{e}");
                    HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Ok(_) => HttpResponse::new(StatusCode::CONFLICT),
        Err(_) =>HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    })
}
#[post("/login")]
pub async fn login(state : web::Data<crate::State> , form : web::Json<LoginForm> , req : HttpRequest) -> impl Responder {
    println!("{:?}" , req.cookie("session_id"));
    let mut conn = state.pool.get().await.unwrap();
    match users::table.filter(users::email.eq(&form.email)).first::<User>(&mut conn).await{
        Ok(user) => match verify(&form.password , &user.password){
            Ok(true) => {
                let session = Session::new(user.id.clone());
                let session_id = session.id.clone();
                match diesel::insert_into(sessions::table)
                    .values(session)
                    .execute(&mut conn)
                    .await {
                    Ok(_) => HttpResponse::Ok()
                                .cookie(Cookie::build("session_id" , session_id)
                                .path("/")
                                .same_site(SameSite::Strict)
                                .domain("localhost")
                                .finish())
                                .finish(),
                    Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
            Ok(false) => HttpResponse::new(StatusCode::UNAUTHORIZED),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Err(NotFound) => HttpResponse::new(StatusCode::NOT_FOUND),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}