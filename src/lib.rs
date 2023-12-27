use webhook_flows::{create_endpoint, request_handler, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(headers: Vec<(String, String)>, _qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    // Deserialize JSON body into LoginRequest struct
    let login_request: Result<LoginRequest, _> = serde_json::from_slice(&body);
    
    // Respond based on the deserialization result
    match login_request {
        Ok(request) => {
            // Perform your login logic using request.username and request.password
            let login_success = is_valid_login(&request.username, &request.password);

            // Respond based on the login result
            let resp = if login_success {
                "Login successful!\n"
            } else {
                "Login failed. Please check your username and password.\n"
            };

            send_response(
                200,
                vec![(String::from("content-type"), String::from("text/html"))],
                resp.as_bytes().to_vec(),
            );
        }
        Err(err) => {
            log::error!("Failed to deserialize JSON body: {:?}", err);

            // Respond with an error if JSON deserialization fails
            send_response(
                400,
                vec![(String::from("content-type"), String::from("text/html"))],
                b"Bad Request: Invalid JSON format.\n".to_vec(),
            );
        }
    }
}

// Replace this with your actual login logic
fn is_valid_login(username: &str, password: &str) -> bool {
    // Basic hardcoded validation (replace with real logic)
    username == "admin" && password == "password"
}
