use sparkpost::transmission::{EmailAddress, Message, Options, Recipient, Transmission, TransmissionResponse};
use crate::errors::ServiceError;
use crate::model::Invitation;

lazy_static!{
    static ref API_KEY:String = std::env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set");
}

pub fn send_invitation(invitation:&Invitation)->Result<(),ServiceError>{
    let tm = Transmission::new_eu(API_KEY.as_str());
    let sending_email =
        std::env::var("SENDING_EMAIL_ADDRESS").expect("SENDING_EMAIL_ADDRESS must be set");
    let mut email = Message::new(EmailAddress::new(sending_email, "Let's Organise"));
    let options = Options {
        open_tracking: false,
        click_tracking: false,
        transactional: true,
        sandbox: false,
        inline_css: false,
        start_time: None,
    };
    let recipient: Recipient = invitation.email.as_str().into();
    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"http://localhost:3000/register.html?id={}&email={}\">
         http://localhost:3030/register</a> <br>
         your Invitation expires on <strong>{}</strong>",
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );
    email
        .add_recipient(recipient)
        .options(options)
        .subject("You have been invited to join Simple-Auth-Server Rust")
        .html(email_body);
    let result = tm.send(&email);
    match result {
        Ok(res)=> match res{
            TransmissionResponse::ApiResponse(api_res)=>{
                println!("API Response: \n {:#?}", api_res);
                Ok(())
            }
            TransmissionResponse::ApiError(errors)=>{
                println!("Response Errors: \n {:#?}", &errors);
                Err(ServiceError::InternalServerError)
            }
        }
        Err(error) => {
            println!("Send Email Error: \n {:#?}", error);
            Err(ServiceError::InternalServerError)
        }
    }

}