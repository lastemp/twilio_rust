use serde::Deserialize;

// Requests

#[derive(Debug)]
pub struct ChannelMessage {
    _sender: MessageSender,   // The message sender.
    _content: MessageContent, // The message content.
    _to: String,              // The recipient's phone number (in E.164 format) or channel address.
}

impl ChannelMessage {
    pub fn new(
        _sender: MessageSender,
        _content: MessageContent,
        _to: String,
    ) -> Result<Self, String> {
        match &_sender {
            MessageSender::From(_from) => {
                if _from.is_empty() || _from.replace(" ", "").trim().len() == 0 {
                    return Err(String::from("message sender is empty"));
                }
            }
            MessageSender::MessagingServiceSid(messaging_service_sid) => {
                if messaging_service_sid.is_empty()
                    || messaging_service_sid.replace(" ", "").trim().len() == 0
                {
                    return Err(String::from("messaging service sid is empty"));
                }
            }
        }

        match &_content {
            MessageContent::Body(_body) => {
                if _body.is_empty() || _body.replace(" ", "").trim().len() == 0 {
                    return Err(String::from("message body is empty"));
                }
                if _body.trim().len() > 1600 {
                    return Err(String::from("message body has exceeded max length"));
                }
            }
            MessageContent::ContentSid(content_sid) => {
                if content_sid.is_empty() || content_sid.replace(" ", "").trim().len() == 0 {
                    return Err(String::from("message content sid is empty"));
                }
            }
            MessageContent::MediaUrl(media_url) => {
                if media_url.is_empty() || media_url.replace(" ", "").trim().len() == 0 {
                    return Err(String::from("message media_url is empty"));
                }
            }
        }

        if _to.is_empty() || _to.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message recipient is empty"));
        }

        Ok(Self {
            _sender,
            _content,
            _to,
        })
    }
    pub fn get_sender(&self) -> &MessageSender {
        let _sender = &self._sender;
        _sender
    }
    pub fn get_content(&self) -> &MessageContent {
        let _content = &self._content;
        _content
    }
    pub fn get_recipient(&self) -> String {
        let _to = &self._to;
        _to.to_string()
    }
}

#[derive(Debug)]
pub struct FetchMessage {
    _sid: String,
}

impl FetchMessage {
    pub fn new(_sid: String) -> Result<Self, String> {
        if _sid.is_empty() || _sid.replace(" ", "").trim().len() == 0 {
            return Err(String::from("sid is empty"));
        }
        Ok(Self { _sid })
    }
    pub fn get_sid(&self) -> String {
        let _sid = &self._sid;
        _sid.to_string()
    }
}

#[derive(Debug)]
pub struct FetchMediaMessage {
    message_sid: String, // The SID of the Message resource that is associated with the Media resource.
    _sid: String,        // The unique identifier of the to-be-deleted Media resource.
}

impl FetchMediaMessage {
    pub fn new(message_sid: String, _sid: String) -> Result<Self, String> {
        if message_sid.is_empty() || message_sid.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message_sid is empty"));
        }
        if _sid.is_empty() || _sid.replace(" ", "").trim().len() == 0 {
            return Err(String::from("sid is empty"));
        }
        Ok(Self { message_sid, _sid })
    }
    pub fn get_message_sid(&self) -> String {
        let message_sid = &self.message_sid;
        message_sid.to_string()
    }
    pub fn get_sid(&self) -> String {
        let _sid = &self._sid;
        _sid.to_string()
    }
}

#[derive(Debug)]
pub struct FetchMultipleMediaMessage {
    message_sid: String, // The SID of the Message resource that is associated with the Media resource.
}

impl FetchMultipleMediaMessage {
    pub fn new(message_sid: String) -> Result<Self, String> {
        if message_sid.is_empty() || message_sid.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message_sid is empty"));
        }

        Ok(Self { message_sid })
    }
    pub fn get_message_sid(&self) -> String {
        let message_sid = &self.message_sid;
        message_sid.to_string()
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

#[derive(Debug)]
pub struct DeleteMediaMessage {
    message_sid: String, // The SID of the Message resource that is associated with the Media resource.
    _sid: String,        // The unique identifier of the to-be-deleted Media resource.
}

impl DeleteMediaMessage {
    pub fn new(message_sid: String, _sid: String) -> Result<Self, String> {
        Ok(Self { message_sid, _sid })
    }
    pub fn get_message_sid(&self) -> String {
        let message_sid = &self.message_sid;
        message_sid.to_string()
    }
    pub fn get_sid(&self) -> String {
        let _sid = &self._sid;
        _sid.to_string()
    }
}

#[derive(Debug)]
pub enum MessageContent {
    Body(String),
    ContentSid(String),
    MediaUrl(String),
}

#[derive(Debug)]
pub enum MessageSender {
    From(String),
    MessagingServiceSid(String),
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
struct MessageTags {
    campaign_name: Option<String>,
    message_type: Option<String>,
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
    tags: Option<MessageTags>,
    to: Option<String>,
    uri: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultMultipleMessage {
    end: Option<u32>,
    first_page_uri: Option<String>,
    next_page_uri: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
    previous_page_uri: Option<String>,
    messages: Vec<ResultMessage>,
    start: Option<u32>,
    uri: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultMediaMessage {
    account_sid: Option<String>,
    content_type: Option<String>,
    date_created: Option<String>,
    date_updated: Option<String>,
    parent_sid: Option<String>,
    sid: Option<String>,
    uri: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultMultipleMediaMessage {
    end: Option<u32>,
    first_page_uri: Option<String>,
    last_page_uri: Option<String>,
    media_list: Vec<ResultMediaMessage>,
    next_page_uri: Option<String>,
    num_pages: Option<u32>,
    page: Option<u32>,
    page_size: Option<u32>,
    previous_page_uri: Option<String>,
    start: Option<u32>,
    total: Option<u32>,
    uri: Option<String>,
}
