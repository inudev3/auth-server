use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use r2d2::Pool;
use serde::Deserialize;

use crate::email_service::send_invitation;
use crate::model::{Invitation, PgPool};



#[derive(Deserialize)]
pub struct InviationData{
    pub emali:String
}
pub async fn post_invitation(invitation_data:web::Json<InviationData>, pool:web::Data<PgPool>)->Result<HttpResponse, actix_web::Error>{
    web::block(move||create_invitation(invitation_data.into_inner().emali,pool)).await?;
    Ok(HttpResponse::Ok().finish())
}
fn create_invitation(email:String,pool:web::Data<PgPool>)->Result<(),crate::errors::ServiceError>{
    let invitation = dbg!(query(email, pool)?);
    send_invitation(&invitation)
}
fn query(email:String, pool:web::Data<PgPool>)->Result<Invitation, crate::errors::ServiceError>{
    use crate::schema::invitations::dsl::invitations;
    let new_invitation:Invitation = email.into();
    let mut conn = pool.get()?;
    let inserted_inviation = diesel::insert_into(invitations)
        .values(new_invitation)
        .get_result(&mut conn)?;
    Ok(inserted_inviation)
}