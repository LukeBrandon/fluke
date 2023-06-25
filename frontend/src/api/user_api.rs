use super::types::{ErrorResponse, User, UserLoginResponse, UserResponse};
use gloo_net::http::{ Request, RequestCredentials };

pub async fn api_register_user(user_data: &str) -> Result<User, String> {
    let response = match Request::post("http://127.0.0.1:8000/register")
        .header("Content-Type", "application/json")
        .body(user_data)
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

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_login_user(credentials: &str) -> Result<UserLoginResponse, String> {
    let response = match Request::post("http://127.0.0.1:8000/login")
        .header("Content-Type", "application/json")
        .body(credentials.clone()) // clone credentials so we can log it
        .send()
        .await
    {
        Ok(res) => {
            // Log the credentials to console
            log::debug!("Sent credentials: {}", credentials);
            res
        },
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() == 401 {
        let body = response.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
        log::debug!("{:?}", body);
        return Err(body);
    }

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }
   
    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
pub async fn api_user_info() -> Result<User, String> {
    let response = match Request::get("http://127.0.0.1:8000/get_user")
        .credentials(RequestCredentials::Include)
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

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_logout_user() -> Result<(), String> {
    let response = match Request::get("http://localhost:8000/api/auth/logout")
        .credentials(RequestCredentials::Include)
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

    Ok(())
}
