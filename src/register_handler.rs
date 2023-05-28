use actix_web::{HttpResponse, web};
use diesel::pg::Pg;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use crate::errors::ServiceError;
use crate::model::{Invitation, PgPool, SlimUser, User};
use crate::utils::hash_password;

#[derive(Debug,Deserialize)]
pub struct UserData{
    pub email:String,
    pub password:String,
}
pub async fn register_user(
    invitation_id:web::Path<String>,
    user_data:web::Json<UserData>,
    pool:web::Data<PgPool>
)->Result<HttpResponse, actix_web::Error>{
    let user = web::block(move||query(invitation_id.into_inner(), user_data.into_inner(), pool)).await??;
    Ok(HttpResponse::Ok().json(&user))
}
fn query(
    invitation_id:String,
    user_data:UserData,
    pool: web::Data<PgPool>
)->Result<SlimUser, crate::errors::ServiceError>{
    use crate::schema::invitations::dsl::{invitations,email,id};
    use crate::schema::users::dsl::users;
    let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;
    let mut conn = pool.get()?;
    invitations
        .filter(id.eq(invitation_id))
        .filter(email.eq(&user_data.email))
        .load::<Invitation>(&mut conn)
        .map_err(|_db_error|ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut result|{
            if let Some(invitation) = result.pop(){
                if invitation.expires_at > chrono::Local::now().naive_local(){
                    let password:String = hash_password(&user_data.password)?;
                    let user = User::from_details(invitation.email, password);
                    let inserted_user:User = diesel::insert_into(users).values(&user).get_result(&mut conn)?;
                    dbg!(&inserted_user);
                    return Ok(inserted_user.into())
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })

}
