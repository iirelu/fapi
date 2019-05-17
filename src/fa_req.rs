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
            .header("Cookie", "b=fdeea4b8-6849-4cde-b18b-f716fd7334df; a=95b1300d-0e12-4314-8d6b-5c202b974caa;")
            .send()
    }
}
