mod customer_info;
mod login;
mod summary;

use crate::customer_info::CustomerInfo;
use crate::login::{auth_request, keyboard_req, mount_password, prepare_key_map};
use crate::summary::Summary;
use reqwest::Client;
use reqwest::Result;
use uuid::Uuid;

pub struct RicoClient {
    inner: Client,
}

impl RicoClient {
    pub async fn login(username: String, password: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.135 Safari/537.36")
            .cookie_store(true)
            .build()
            .expect("Failed to build reqwest client.");

        let session_id = Uuid::new_v4();
        let kb_res = keyboard_req(&client, &session_id, &username).await?;
        let key_map = prepare_key_map(&kb_res);
        let pwd = mount_password(password, key_map);

        auth_request(&client, &username, &pwd, &session_id, &kb_res.token).await?;

        Ok(RicoClient { inner: client })
    }

    pub async fn customer_info(&self) -> Result<CustomerInfo> {
        self.inner
            .get("https://www.rico.com.vc/api/customer-info")
            .send()
            .await?
            .json::<CustomerInfo>()
            .await
    }

    pub async fn summary(&self) -> Result<Summary> {
        self.inner
            .get("https://www.rico.com.vc/api/finance/summary-position/")
            .send()
            .await?
            .json::<Summary>()
            .await
    }
}
