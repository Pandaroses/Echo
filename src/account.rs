use actix_web::{get, Responder, HttpResponse , web::{self, Redirect}, HttpRequest, http::StatusCode, HttpResponseBuilder};
use diesel_async::RunQueryDsl;
use diesel::{prelude::*, result::Error::NotFound};
use serde::Serialize;

use crate::{schemas::{sessions, users}, models::User};

#[derive(Serialize)]
struct AccountDetails {
    email : String
}

#[get("/details")]
pub async fn details(state : web::Data<crate::State> , req : HttpRequest) -> impl Responder{
    let mut conn = state.pool.get().await.unwrap();
    println!("{:?}" , req.cookie("session_id").unwrap().value());
    match sessions::table
        .inner_join(users::table)
        .filter(sessions::id.eq(match req.cookie("session_id") {
            Some(cookie) => cookie.value().to_string(),
            None => "".to_string(),
        }))
        .select(User::as_select())
        .first::<User>(&mut conn).await {
        Ok(user) => {
            let details = AccountDetails { email : user.email };
            HttpResponseBuilder::new(StatusCode::OK).json(web::Json(details))
        },
        Err(NotFound) => return HttpResponse::new(StatusCode::NOT_FOUND),
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}