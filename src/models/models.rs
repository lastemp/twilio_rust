use serde::Deserialize;

// Requests

#[derive(Debug)]
pub struct SmsMessage {
    _from: String, // The sender's phone number (in E.164 format), alphanumeric sender ID, Wireless SIM, short code, or channel address.
    _body: String, // The text content of the message.
    _to: String,   // The recipient's phone number (in E.164 format) or channel address.
}

impl SmsMessage {
    pub fn new(_from: String, _body: String, _to: String) -> Result<Self, String> {
        if _from.is_empty() || _from.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message sender is empty"));
        }

        if _body.is_empty() || _body.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message body is empty"));
        }

        if _to.is_empty() || _to.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message recipient is empty"));
        }

        Ok(Self { _from, _body, _to })
    }
    pub fn get_sender(&self) -> String {
        let _from = &self._from;
        _from.to_string()
    }
    pub fn get_body(&self) -> String {
        let _body = &self._body;
        _body.to_string()
    }
    pub fn get_recipient(&self) -> String {
        let _to = &self._to;
        _to.to_string()
    }
}

#[derive(Debug)]
pub struct WhatsappMessage {
    _from: String, // The sender's phone number (in E.164 format), alphanumeric sender ID, Wireless SIM, short code, or channel address.
    _body: String, // The text content of the message.
    _to: String,   // The recipient's phone number (in E.164 format) or channel address.
}

impl WhatsappMessage {
    pub fn new(_from: String, _body: String, _to: String) -> Result<Self, String> {
        if _from.is_empty() || _from.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message sender is empty"));
        }

        if _body.is_empty() || _body.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message body is empty"));
        }

        if _to.is_empty() || _to.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message recipient is empty"));
        }

        Ok(Self { _from, _body, _to })
    }
    pub fn get_sender(&self) -> String {
        let _from = &self._from;
        _from.to_string()
    }
    pub fn get_body(&self) -> String {
        let _body = &self._body;
        _body.to_string()
    }
    pub fn get_recipient(&self) -> String {
        let _to = &self._to;
        _to.to_string()
    }
}

#[derive(Debug)]
pub struct FetchMessage {
    _sid: Option<String>,
}

impl FetchMessage {
    pub fn new(_sid: Option<String>) -> Result<Self, String> {
        Ok(Self { _sid })
    }
    pub fn get_sid(&self) -> &Option<String> {
        let _sid = &self._sid;
        _sid
    }
}

#[derive(Debug)]
pub struct UpdateMessage {
    _sid: String,
}

impl UpdateMessage {
    pub fn new(_sid: String) -> Result<Self, String> {
        Ok(Self { _sid })
    }
    pub fn get_sid(&self) -> String {
        let _sid = &self._sid;
        _sid.to_string()
    }
}

#[derive(Debug)]
pub struct DeleteMessage {
    _sid: String,
}

impl DeleteMessage {
    pub fn new(_sid: String) -> Result<Self, String> {
        Ok(Self { _sid })
    }
    pub fn get_sid(&self) -> String {
        let _sid = &self._sid;
        _sid.to_string()
    }
}

// Response

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct MessageMedia {
    media: Option<String>,
    feedback: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultMessage {
    account_sid: Option<String>,
    api_version: Option<String>,
    body: Option<String>,
    date_created: Option<String>,
    date_sent: Option<String>,
    date_updated: Option<String>,
    direction: Option<String>,
    error_code: Option<u32>,
    error_message: Option<String>,
    from: Option<String>,
    messaging_service_sid: Option<String>,
    num_media: Option<String>,
    num_segments: Option<String>,
    price: Option<String>,
    price_unit: Option<String>,
    sid: Option<String>,
    status: Option<String>,
    subresource_uris: Option<MessageMedia>,
    to: Option<String>,
    uri: Option<String>,
}
