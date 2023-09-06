mod sms {
    pub mod create_sms;
}

mod whatsapp {
    pub mod create_whatsapp;
}

mod fetch_message {
    pub mod fetch_message;
}

mod fetch_media_message {
    pub mod fetch_media_message;
}

mod update_message {
    pub mod update_message;
}

mod delete_message {
    pub mod delete_message;
}

mod delete_media_message {
    pub mod delete_media_message;
}

mod util {
    pub mod util;
}

pub mod models {
    pub mod models;
}

use models::models::{
    ChannelMessage, DeleteMediaMessage, DeleteMessage, FetchMediaMessage, FetchMessage,
    FetchMultipleMediaMessage, ResultMediaMessage, ResultMessage, ResultMultipleMediaMessage,
    UpdateMessage,
};

const BASE_MESSAGE_URL_PROD: &str = "https://api.twilio.com/2010-04-01/Accounts/";
const MESSAGES_JSON: &str = "/Messages.json";

#[derive(Debug)]
pub struct Twilio {
    account_sid: String,
    auth_token: String,
    message_url: String,
}

impl Twilio {
    pub fn new(account_sid: String, auth_token: String) -> Result<Self, String> {
        if account_sid.is_empty() || account_sid.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account sid is empty"));
        }

        if auth_token.is_empty() || auth_token.replace(" ", "").trim().len() == 0 {
            return Err(String::from("auth token is empty"));
        }

        let mut message_url = String::from("");
        message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
        message_url.push_str(&account_sid);
        message_url.push_str(&MESSAGES_JSON.to_string());

        Ok(Self {
            account_sid,
            auth_token,
            message_url,
        })
    }

    // SMS
    pub async fn create_sms(
        &self,
        _message: ChannelMessage,
    ) -> std::result::Result<ResultMessage, String> {
        let _sender = _message.get_sender();
        let _content = _message.get_content();
        let _to = _message.get_recipient();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;
        let api_url = &self.message_url;

        let _output = sms::create_sms::create(
            _sender,
            _content,
            _to,
            account_sid.to_string(),
            auth_token.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // SMS, Whatsapp
    pub async fn fetch_message(
        &self,
        _message: FetchMessage,
    ) -> std::result::Result<ResultMessage, String> {
        let _sid = _message.get_sid();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;

        let messages_tag = String::from("/Messages/");
        let json_tag = String::from(".json");

        // Fetch a Message resource
        // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json
        let mut message_url = String::from("");
        message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
        message_url.push_str(&account_sid);
        message_url.push_str(&messages_tag);
        message_url.push_str(&_sid);
        message_url.push_str(&json_tag);

        let api_url = message_url;

        let _output = fetch_message::fetch_message::fetch(
            account_sid.to_string(),
            auth_token.to_string(),
            api_url,
        );

        let _result = _output.await;

        _result
    }

    // SMS, Whatsapp
    pub async fn fetch_multiple_message(&self) -> std::result::Result<ResultMessage, String> {
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;
        // Read multiple Message resources
        // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages.json
        let api_url = &self.message_url;

        let _output = fetch_message::fetch_message::fetch(
            account_sid.to_string(),
            auth_token.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // SMS, Whatsapp
    pub async fn fetch_media_message(
        &self,
        _message: FetchMediaMessage,
    ) -> std::result::Result<ResultMediaMessage, String> {
        let message_sid = _message.get_message_sid();
        let _sid = _message.get_sid();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;

        let messages_tag = String::from("/Messages/");
        let media_tag = String::from("/Media/");
        let json_tag = String::from(".json");

        // Fetch a media Message resource
        // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json
        let mut message_url = String::from("");
        message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
        message_url.push_str(&account_sid);
        message_url.push_str(&messages_tag);
        message_url.push_str(&message_sid);
        message_url.push_str(&media_tag);
        message_url.push_str(&_sid);
        message_url.push_str(&json_tag);

        let api_url = message_url;

        let _output = fetch_media_message::fetch_media_message::fetch(
            account_sid.to_string(),
            auth_token.to_string(),
            api_url,
        );

        let _result = _output.await;

        _result
    }

    // SMS, Whatsapp
    pub async fn fetch_multiple_media_message(
        &self,
        _message: FetchMultipleMediaMessage,
    ) -> std::result::Result<ResultMultipleMediaMessage, String> {
        let message_sid = _message.get_message_sid();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;

        let messages_tag = String::from("/Messages/");
        let media_tag = String::from("/Media");
        let json_tag = String::from(".json");

        // Fetch a media Message resource
        // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media.json
        let mut message_url = String::from("");
        message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
        message_url.push_str(&account_sid);
        message_url.push_str(&messages_tag);
        message_url.push_str(&message_sid);
        message_url.push_str(&media_tag);
        message_url.push_str(&json_tag);

        let api_url = message_url;

        let _output = fetch_media_message::fetch_media_message::fetch_multiple(
            account_sid.to_string(),
            auth_token.to_string(),
            api_url,
        );

        let _result = _output.await;

        _result
    }

    // Whatsapp
    pub async fn create_whatsapp(
        &self,
        _message: ChannelMessage,
    ) -> std::result::Result<ResultMessage, String> {
        let _sender = _message.get_sender();
        let _content = _message.get_content();
        let _to = _message.get_recipient();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;
        let api_url = &self.message_url;

        let _output = whatsapp::create_whatsapp::create(
            _sender,
            _content,
            _to,
            account_sid.to_string(),
            auth_token.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // SMS, Whatsapp
    pub async fn update_message(
        &self,
        _message: UpdateMessage,
    ) -> std::result::Result<ResultMessage, String> {
        let _sid = _message.get_sid();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;

        let messages_tag = String::from("/Messages/");
        let json_tag = String::from(".json");

        // Update a Message resource
        // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json
        let mut message_url = String::from("");
        message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
        message_url.push_str(&account_sid);
        message_url.push_str(&messages_tag);
        message_url.push_str(&_sid);
        message_url.push_str(&json_tag);

        let api_url = message_url;

        let _output = update_message::update_message::update(
            account_sid.to_string(),
            auth_token.to_string(),
            api_url,
        );

        let _result = _output.await;

        _result
    }

    // SMS, Whatsapp
    pub async fn delete_message(
        &self,
        _message: DeleteMessage,
    ) -> std::result::Result<bool, String> {
        let _sid = _message.get_sid();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;

        let messages_tag = String::from("/Messages/");
        let json_tag = String::from(".json");

        // Delete a Message resource
        // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json
        let mut message_url = String::from("");
        message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
        message_url.push_str(&account_sid);
        message_url.push_str(&messages_tag);
        message_url.push_str(&_sid);
        message_url.push_str(&json_tag);

        let api_url = message_url;

        let _output = delete_message::delete_message::delete(
            account_sid.to_string(),
            auth_token.to_string(),
            api_url,
        );

        let _result = _output.await;

        _result
    }

    // SMS, Whatsapp
    pub async fn delete_media_message(
        &self,
        _message: DeleteMediaMessage,
    ) -> std::result::Result<bool, String> {
        let message_sid = _message.get_message_sid();
        let _sid = _message.get_sid();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;

        let messages_tag = String::from("/Messages/");
        let media_tag = String::from("/Media/");
        let json_tag = String::from(".json");

        // Delete a media Message resource
        // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json
        let mut message_url = String::from("");
        message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
        message_url.push_str(&account_sid);
        message_url.push_str(&messages_tag);
        message_url.push_str(&message_sid);
        message_url.push_str(&media_tag);
        message_url.push_str(&_sid);
        message_url.push_str(&json_tag);

        let api_url = message_url;

        let _output = delete_media_message::delete_media_message::delete(
            account_sid.to_string(),
            auth_token.to_string(),
            api_url,
        );

        let _result = _output.await;

        _result
    }
}

#[cfg(test)]
mod tests {
    use crate::models::models::{MessageContent, MessageSender};

    use super::*;

    #[test]
    fn test_twilio() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

        let _result = Twilio::new(account_sid, auth_token);
        assert_eq!(_result.is_ok(), true);
    }

    #[test]
    fn test_sms_message() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

        let _result = Twilio::new(account_sid, auth_token);

        if let Ok(_twilio) = _result {
            let _from = String::from("+15******661");
            let _sender = MessageSender::From(_from);
            let _body = String::from("Hi there"); // The text content of the message.
            let _content = MessageContent::Body(_body);
            let _to = String::from("+15******310");

            let _result = ChannelMessage::new(_sender, _content, _to);
            assert_eq!(_result.is_ok(), true);
        }
    }

    #[tokio::test]
    async fn test_create_sms_message() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

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
                let _output = _twilio.create_sms(_message);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }

    #[tokio::test]
    async fn test_create_whatsapp_message() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

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
                let _output = _twilio.create_whatsapp(_message);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_one_message() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

        let _result = Twilio::new(account_sid, auth_token);

        if let Ok(_twilio) = _result {
            let _sid = String::from("***"); // The SID of the Message resource to be fetched

            let _result = FetchMessage::new(_sid);

            if let Ok(_message) = _result {
                let _output = _twilio.fetch_message(_message);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_multiple_messages() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

        let _result = Twilio::new(account_sid, auth_token);

        if let Ok(_twilio) = _result {
            let _output = _twilio.fetch_multiple_message();
            let _result = _output.await;
            assert_eq!(_result.is_ok(), true);
        }
    }

    #[tokio::test]
    async fn test_update_message() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

        let _result = Twilio::new(account_sid, auth_token);

        if let Ok(_twilio) = _result {
            let _sid = String::from("***"); // The SID of the Message resource to be updated

            let _result = UpdateMessage::new(_sid);

            if let Ok(_message) = _result {
                let _output = _twilio.update_message(_message);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }

    #[tokio::test]
    async fn test_delete_message() {
        let account_sid = String::from("***");
        let auth_token = String::from("***");

        let _result = Twilio::new(account_sid, auth_token);

        if let Ok(_twilio) = _result {
            let _sid = String::from("***"); // The SID of the Message resource to be deleted

            let _result = DeleteMessage::new(_sid);

            if let Ok(_message) = _result {
                let _output = _twilio.delete_message(_message);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }
}
