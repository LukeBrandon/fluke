use super::types::ErrorResponse;
use crate::api::types;
use reqwasm::http::Request;

pub async fn api_create_user(user_data: &str) -> Result<types::UserModel, String> {
    let response = match Request::post("http://127.0.0.1:8000/users")
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => {
            log::debug!("Body sent: {}", user_data);
            log::info!("Response: {:?}", res);
            res
        }
        Err(e) => {
            log::error!("Error making request: {:?}", e);
            return Err("Failed to make request".to_string());
        }
    };

    let status: u16 = response.status();
    let body: String = response
        .text()
        .await
        .unwrap_or_else(|_| "Failed to read response body".to_string());

    if status == 401 {
        log::debug!("{:?}", body);
        return Err(body);
    }

    if status != 200 {
        let error_response = serde_json::from_str::<ErrorResponse>(&body);
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            log::error!("Unexpected API error. Status: {}. Body: {}", status, body);
            return Err(format!("API error: {}", status));
        }
    }

    match serde_json::from_str::<types::UserModel>(&body) {
        Ok(data) => Ok(data),
        Err(e) => {
            log::error!("Failed to parse response. Body: {}. Error: {:?}", body, e);
            Err("Failed to parse response".to_string())
        }
    }
}

// pub async fn api_user_info() -> Result<User, String> {
//     let response = match Request::get("http://127.0.0.1:8000/get_user").send().await {
//         Ok(res) => res,
//         Err(e) => {
//             log::info!("Failure here: {:?}", e);
//             return Err("Failed to make request".to_string());
//         }
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     let res_json = response.json::<UserResponse>().await;
//     match res_json {
//         Ok(data) => Ok(data.data.user),
//         Err(_) => Err("Failed to parse response".to_string()),
//     }
// }

// pub async fn api_logout_user() -> Result<(), String> {
//     let response = match Request::get("http://localhost:8000/api/auth/logout")
//         .credentials(RequestCredentials::Include)
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     Ok(())
// }

// pub async fn api_register_user(user_data: &str) -> Result<UserResponse, String> {
//     let response = match Request::post("http://127.0.0.1:8000/users/signup")
//         .header("Content-Type", "application/json")
//         .send()
//         .await
//     {
//         Ok(res) => Ok(res.json()),
//         Err(e) => {
//             return Err("Failed to make request".to_string());
//         }
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     let res_json = response.json::<UserResponse>().await;
//     match res_json {
//         Ok(data) => Ok(data.data.user),
//         Err(_) => Err("Failed to parse response".to_string()),
//     }
// }
