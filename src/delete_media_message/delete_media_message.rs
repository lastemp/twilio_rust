use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn delete(
    account_sid: String,
    auth_token: String,
    api_url: String,
) -> std::result::Result<bool, String> {
    let client = reqwest::Client::new();
    let res = client
        .delete(api_url)
        .basic_auth(account_sid, Some(auth_token)) // Basic authentication
        .headers(build_headers())
        .send()
        .await;

    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::NO_CONTENT => {
                return Ok(true);
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
