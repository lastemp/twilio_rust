use crate::models::models::{MessageContent, MessageSender, ResultMessage};
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn create(
    _sender: &MessageSender,
    _content: &MessageContent,
    _to: String,
    account_sid: String,
    auth_token: String,
    api_url: String,
) -> std::result::Result<ResultMessage, String> {
    let mut params = Vec::new();

    if let MessageSender::From(_from) = _sender {
        params.push(("From", _from.to_string()));
    } else if let MessageSender::MessagingServiceSid(messaging_service_sid) = _sender {
        params.push(("MessagingServiceSid", messaging_service_sid.to_string()));
    }

    if let MessageContent::Body(_body) = _content {
        params.push(("Body", _body.to_string()));
    } else if let MessageContent::ContentSid(content_sid) = _content {
        params.push(("ContentSid", content_sid.to_string()));
    } else if let MessageContent::MediaUrl(media_url) = _content {
        params.push(("MediaUrl", media_url.to_string()));
    }

    params.push(("To", _to));

    let client = reqwest::Client::new();
    let res = client
        .post(api_url)
        .basic_auth(account_sid, Some(auth_token)) // Basic authentication
        .headers(build_headers())
        .form(&params)
        .send()
        .await;

    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                match response.json::<ResultMessage>().await {
                    Ok(result_message) => {
                        // Handle success case
                        return Ok(result_message);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
