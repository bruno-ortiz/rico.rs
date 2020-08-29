use reqwest::Result;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) async fn auth_request(
    client: &Client,
    username: &str,
    pwd: &str,
    session_id: &Uuid,
    token: &str,
) -> Result<Response> {
    let body = json!({
            "password": &pwd,
            "sessionId": session_id,
            "token": token,
            "username": &username
    });

    let req = client
        .post("https://www.rico.com.vc/api/oauth/")
        .header("origin", "https://www.rico.com.vc")
        .header("referer", "https://www.rico.com.vc/login/")
        .json(&body)
        .build()
        .unwrap();

    client.execute(req).await
}

pub(crate) async fn keyboard_req(
    client: &Client,
    session_id: &Uuid,
    username: &str,
) -> Result<KeyboardResponse> {
    let body = json!({
        "sessionId" : session_id,
        "username": username
    });

    let req = client
        .post("https://www.rico.com.vc/api/oauth/keyboard/")
        .header("origin", "https://www.rico.com.vc")
        .header("referer", "https://www.rico.com.vc/login/")
        .json(&body)
        .build()?;

    client.execute(req).await?.json::<KeyboardResponse>().await
}

pub(crate) fn prepare_key_map(res: &KeyboardResponse) -> HashMap<String, String> {
    let mut key_map = HashMap::new();

    for (key, value) in &res.keys {
        for n in value {
            key_map.insert(n.clone(), key.clone());
        }
    }
    key_map
}

pub(crate) fn mount_password(password: String, key_map: HashMap<String, String>) -> String {
    password
        .chars()
        .map(|c| key_map.get(&c.to_string()).unwrap().clone())
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct KeyboardResponse {
    pub(crate) keys: HashMap<String, Vec<String>>,
    pub(crate) token: String,
}
