use twilio_rust::models::models::FetchMediaMessage;
use twilio_rust::Twilio;

pub async fn fetch_media_message(account_sid: String, auth_token: String) {
    let _result = Twilio::new(account_sid, auth_token);

    if let Ok(_twilio) = _result {
        let message_sid = String::from("***"); // The SID of the Message resource that is associated with the Media resource.
        let _sid = String::from("***"); // The SID of the Message resource to be fetched

        let _result = FetchMediaMessage::new(message_sid, _sid);

        if let Ok(_message) = _result {
            let _output = _twilio.fetch_media_message(_message);
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
