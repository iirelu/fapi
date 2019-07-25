use reqwest::{Client, Response};
use reqwest::Result as ReqResult;

pub struct FaReq {
    client: Client,
}

impl FaReq {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get_submission_page(&self, id: u64) -> ReqResult<Response> {
        self.client.get(&format!("https://www.furaffinity.net/view/{}", id))
            .header("Cookie", include_str!("LOGIN_COOKIE"))
            .send()
    }
}
