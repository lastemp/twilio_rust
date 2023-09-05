mod sms {
    pub mod create_sms;
}

mod util {
    pub mod util;
}

pub mod models {
    pub mod models;
}

use models::models::{ResultSmsMessage, SmsMessage};

const SMS_MESSAGES_JSON: &str = "/Messages.json";
const SMS_URL_PROD: &str = "https://api.twilio.com/2010-04-01/Accounts/";

#[derive(Debug)]
pub struct Twilio {
    account_sid: String,
    auth_token: String,
    sms_url: String,
}

impl Twilio {
    pub fn new(account_sid: String, auth_token: String) -> Result<Self, String> {
        if account_sid.is_empty() || account_sid.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account sid is empty"));
        }

        if auth_token.is_empty() || auth_token.replace(" ", "").trim().len() == 0 {
            return Err(String::from("auth token is empty"));
        }

        let mut sms_url = String::from("");
        sms_url.push_str(&SMS_URL_PROD.to_string());
        sms_url.push_str(&account_sid);
        sms_url.push_str(&SMS_MESSAGES_JSON.to_string());

        Ok(Self {
            account_sid,
            auth_token,
            sms_url,
        })
    }

    // SMS
    pub async fn create(
        &self,
        sms_message: SmsMessage,
    ) -> std::result::Result<ResultSmsMessage, String> {
        let _from = sms_message.get_sender();
        let _body: String = sms_message.get_body();
        let _to = sms_message.get_recipient();
        let account_sid = &self.account_sid;
        let auth_token = &self.auth_token;
        let api_url = &self.sms_url;

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

        if let Ok(twilio) = _result {
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

        if let Ok(twilio) = _result {
            let _from = String::from("+15******661");
            let _body = String::from("Hi there");
            let _to = String::from("+15******310");

            let _result = SmsMessage::new(_from, _body, _to);

            if let Ok(sms_message) = _result {
                let _output = twilio.create(sms_message);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }
}
