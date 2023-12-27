use webhook_flows::{create_endpoint, request_handler, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde_json::Value;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    // Extract username and password from query parameters
    let username = qry.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let password = qry.get("password").and_then(|v| v.as_str()).unwrap_or("");

    // Perform your login logic (replace this with your actual logic)
    let login_success = is_valid_login(username, password);

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

// Replace this with your actual login logic
fn is_valid_login(username: &str, password: &str) -> bool {
    // Basic hardcoded validation (replace with real logic)
    username == "admin" && password == "password"
}
