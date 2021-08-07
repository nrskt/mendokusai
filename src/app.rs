use fantoccini::{Client, ClientBuilder};

use crate::MendokusaiError;

pub struct Mendokusai {
    client: Client,
}

impl Mendokusai {
    pub async fn new(driver: &str) -> Result<Self, MendokusaiError> {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let client = ClientBuilder::rustls().connect(driver).await?;
        Ok(Self { client })
    }

    pub fn client(&mut self) -> &mut Client {
        &mut self.client
    }
}
