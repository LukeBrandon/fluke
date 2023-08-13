use super::types::{ErrorResponse, Message};
use gloo_net::http::Request;

pub async fn api_get_messages() -> Result<Vec<Message>, String> {
    let response = match Request::get("http://127.0.0.1:8000/messages").send().await {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<Vec<Message>>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
