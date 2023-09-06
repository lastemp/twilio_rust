use twilio_rust::models::models::{ChannelMessage, MessageContent, MessageSender};
use twilio_rust::Twilio;

pub async fn create_whatsapp_message(account_sid: String, auth_token: String) {
    let _result = Twilio::new(account_sid, auth_token);

    if let Ok(_twilio) = _result {
        let _from = String::from("whatsapp:+15******661");
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
        let _to = String::from("whatsapp:+15******310");

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
}
