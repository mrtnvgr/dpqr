use reqwest::{Client, Response, Result};
use std::collections::HashMap;
use std::env;

const VK_V: &'static str = "5.131";

pub async fn send_pass_to_vk(pass: &str) -> Result<Response> {
    let token = env::var("VK_TOKEN").expect("Token not found");
    let user_id = env::var("VK_USER_ID").expect("Receiver id not found");

    let client = Client::new();
    let url = "https://api.vk.com/method/messages.send";

    let message = format!("[Ещё один прошедший, код: {pass}]");

    let mut params = HashMap::new();
    params.insert("access_token", token.as_str());
    params.insert("v", VK_V);
    params.insert("random_id", "0");
    params.insert("user_id", user_id.as_str());
    params.insert("message", &message);

    client.get(url).query(&params).send().await
}
