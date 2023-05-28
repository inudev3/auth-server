use actix_identity::Identity;
use actix_web::{FromRequest, HttpRequest, HttpResponse, web};
use actix_web::dev::Payload;
use diesel::pg::Pg;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use futures::future::{err, ok, Ready};
use serde::Deserialize;
use crate::errors::ServiceError;
use crate::model::{PgPool, SlimUser, User};
use crate::schema::users::dsl::users;
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData{
    pub email:String,
    pub password:String,
}
pub type LoggedUser = SlimUser;
impl FromRequest for LoggedUser{
    type Error = actix_web::Error;
    type Future = Ready<Result<LoggedUser,actix_web::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner(){
            if let Some(user_json) =identity.identity(){
                if let Ok(user) = serde_json::from_str(&user_json){
                    return ok(user)
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }
}
pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}
pub async fn login(
    auth_data: web::Json<AuthData>,
    id:Identity,
    pool:web::Data<PgPool>
) ->Result<HttpResponse, actix_web::Error>{
    let user = web::block(move||query(auth_data.into_inner(), pool)).await??;
    let user_string = serde_json::to_string(&user)?;
    id.remember(user_string);
    Ok(HttpResponse::Ok().finish())
}
pub async fn get_me(
    logged_user:LoggedUser
)->HttpResponse{
    HttpResponse::Ok().json(logged_user)
}
fn query(auth_data:AuthData, pool:web::Data<PgPool>)->Result<SlimUser, ServiceError>{
    use crate::schema::users::dsl::{users,email};
    let mut conn = pool.get()?;
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(&mut conn)?;
    if let Some(user) = items.pop(){
        if let Ok(matching) = verify(&user.hash, &auth_data.password){
            if matching{
                return Ok(user.into())
            }
        }
    }
    Err(ServiceError::Unauthorized)
}