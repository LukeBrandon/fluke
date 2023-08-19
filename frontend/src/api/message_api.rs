use super::types::{Channel, ErrorResponse, Message};
use reqwasm::http::Request;

pub async fn api_get_messages(channel_id: i64) -> Result<Vec<Message>, String> {
    let uri_builder = format!("http://127.0.0.1:8000/channels/{}/messages", channel_id);
    let response = match Request::get(&uri_builder).send().await {
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

pub async fn api_create_message(
    channel_id: &i64,
    message: &str,
    user_id: i64,
) -> Result<Message, String> {
    let uri_builder = format!("http://localhost:8000/channels/{}/messages", channel_id);
    let body = serde_json::json!({
        "message": message,
        "user_id": user_id
    });

    // Convert the body to a String
    let body_string = match serde_json::to_string(&body) {
        Ok(s) => s,
        Err(_) => return Err("Failed to serialize body to JSON".to_string()),
    };

    let response = match Request::post(&uri_builder)
        .header("Content-Type", "application/json")
        .body(&body_string)
        .send()
        .await
    {
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

    let res_json = response.json::<Message>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_get_channels() -> Result<Vec<Channel>, String> {
    let uri_builder = "http://127.0.0.1:8000/channels";
    let response = match Request::get(&uri_builder).send().await {
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

    let res_json = response.json::<Vec<Channel>>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
