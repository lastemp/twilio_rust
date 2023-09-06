use crate::models::models::{ResultMediaMessage, ResultMultipleMediaMessage};
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn fetch(
    account_sid: String,
    auth_token: String,
    api_url: String,
) -> std::result::Result<ResultMediaMessage, String> {
    let client = reqwest::Client::new();
    let res = client
        .get(api_url)
        .basic_auth(account_sid, Some(auth_token)) // Basic authentication
        .headers(build_headers())
        .send()
        .await;

    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                match response.json::<ResultMediaMessage>().await {
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

pub async fn fetch_multiple(
    account_sid: String,
    auth_token: String,
    api_url: String,
) -> std::result::Result<ResultMultipleMediaMessage, String> {
    let client = reqwest::Client::new();
    let res = client
        .get(api_url)
        .basic_auth(account_sid, Some(auth_token)) // Basic authentication
        .headers(build_headers())
        .send()
        .await;

    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                match response.json::<ResultMultipleMediaMessage>().await {
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
