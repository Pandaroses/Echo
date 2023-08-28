use actix_web::{post , Responder, HttpResponse , web};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct UserForm {
    email : String,
    password : String
}

#[post("/register")]
pub async fn register(data : web::Data<crate::State> , _form : web::Json<UserForm>) -> impl Responder{
    HttpResponse::Ok().body(data.message.clone())
}
#[post("/login")]
pub async fn login(data : web::Data<crate::State> , _form : web::Json<UserForm>) -> impl Responder {
    HttpResponse::Ok()
}