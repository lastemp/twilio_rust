use dotenv::dotenv;
use std::env;

mod create_sms;
mod create_whatsapp;
mod delete_media_message;
mod delete_message;
mod fetch_media_message;
mod fetch_message;
mod fetch_multiple_media_message;
mod fetch_multiple_messages;
mod update_message;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let account_sid = env::var("ACCOUNT_SID").expect("ACCOUNT_SID is not set in .env file");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN is not set in .env file");
    // PART A: TESTS
    // sms
    let x = create_sms::create_sms_message(account_sid, auth_token);

    // whatsapp
    // let x = create_whatsapp::create_whatsapp_message(account_sid, auth_token);

    // let x = fetch_message::fetch_message(account_sid, auth_token);
    // let x = fetch_multiple_messages::fetch_multiple_message(account_sid, auth_token);

    // let x = fetch_media_message::fetch_media_message(account_sid, auth_token);
    // let x = fetch_multiple_media_message::fetch_multiple_media_message(account_sid, auth_token);

    // let x = update_message::update_message(account_sid, auth_token);

    // let x = delete_message::delete_message(account_sid, auth_token);
    // let x = delete_media_message::delete_media_message(account_sid, auth_token);

    x.await;

    // PART B: TESTS

    // This example code for sending sms can be un-commented for testing
    /*
    use twilio_rust::models::models::{ChannelMessage, MessageContent, MessageSender};
    use twilio_rust::Twilio;

    let _result = Twilio::new(account_sid, auth_token);

    if let Ok(_twilio) = _result {
        let _from = String::from("+15******661");
        // let messaging_service_sid = String::from("***");
        let _sender = MessageSender::From(_from);
        //let _sender = MessageSender::MessagingServiceSid(messaging_service_sid);
        let _body = String::from("Hi there");
        // let content_sid = String::from("***");
        // let media_url = String::from("***");
        // "MessageContent::Body": The text content of the message.
        let _content = MessageContent::Body(_body);
        // let _content = MessageContent::ContentSid(content_sid);
        // let _content = MessageContent::MediaUrl(media_url);
        let _to = String::from("+15******310");

        let _result = ChannelMessage::new(_sender, _content, _to);

        if let Ok(_message) = _result {
            let _output = _twilio.create_message(_message);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
            } else if let Err(e) = _result {
                println!("{:?}", e);
            } else {
                println!("Unexpected error occured during processing");
            }
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
    */
}
