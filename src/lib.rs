mod sms {
    pub mod create_sms;
}

mod whatsapp {
    pub mod create_whatsapp;
}

mod fetch_message {
    pub mod fetch_message;
}

mod update_message {
    pub mod update_message;
}

mod delete_message {
    pub mod delete_message;
}

mod util {
    pub mod util;
}

pub mod models {
    pub mod models;
}

use models::models::{
    DeleteMessage, FetchMessage, ResultMessage, SmsMessage, UpdateMessage, WhatsappMessage,
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
        _message: SmsMessage,
    ) -> std::result::Result<ResultMessage, String> {
        let _from = _message.get_sender();
        let _body: String = _message.get_body();
        let _to = _message.get_recipient();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;
        let api_url = &self.message_url;

        let _output = sms::create_sms::create(
            _from,
            _body,
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

        let api_url = if let Some(_sid) = _sid {
            // Fetch a Message resource
            // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json
            let mut message_url = String::from("");
            message_url.push_str(&BASE_MESSAGE_URL_PROD.to_string());
            message_url.push_str(&account_sid);
            message_url.push_str(&messages_tag);
            message_url.push_str(&_sid);
            message_url.push_str(&json_tag);

            message_url
        } else {
            // Read multiple Message resources
            // https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/Messages.json
            let message_url = &self.message_url;
            message_url.to_string()
        };

        let _output = fetch_message::fetch_message::fetch(
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
        _message: WhatsappMessage,
    ) -> std::result::Result<ResultMessage, String> {
        let _from = _message.get_sender();
        let _body: String = _message.get_body();
        let _to = _message.get_recipient();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;
        let api_url = &self.message_url;

        let _output = whatsapp::create_whatsapp::create(
            _from,
            _body,
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
    ) -> std::result::Result<ResultMessage, String> {
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
}

#[cfg(test)]
mod tests {
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
            let _body = String::from("Hi there");
            let _to = String::from("+15******310");

            let _result = SmsMessage::new(_from, _body, _to);
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
            let _body = String::from("Hi there");
            let _to = String::from("+15******310");

            let _result = SmsMessage::new(_from, _body, _to);

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
            let _body = String::from("Hi there");
            let _to = String::from("whatsapp:+15******310");

            let _result = WhatsappMessage::new(_from, _body, _to);

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
            let _sid = Some(String::from("***")); // The SID of the Message resource to be fetched

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
            let _sid = None; // This will enable return of multiple messages

            let _result = FetchMessage::new(_sid);

            if let Ok(_message) = _result {
                let _output = _twilio.fetch_message(_message);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
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
