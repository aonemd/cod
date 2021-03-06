use super::types::{Payload, WriteCommands, WriteResponse};

use serde_json::json;

pub struct SyncApi {
    token: String,
    uri: String,
    http_client: reqwest::Client,
}

impl SyncApi {
    pub fn new(token: String) -> Self {
        Self {
            token,
            uri: format!("{}/sync", super::todoist::BASE_URI),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn read_resources(
        &self,
        resource_types: Option<Vec<&str>>,
    ) -> Result<Payload, Box<dyn std::error::Error>> {
        let resource_types = resource_types.unwrap_or(vec!["all"]);

        let res = self
            .http_client
            .get(&self.uri)
            .query(&[
                ("token", &self.token),
                ("sync_token", &"*".to_string()),
                ("resource_types", &json!(resource_types).to_string()),
            ])
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        let body = res.json::<serde_json::Value>().await?;
        let payload: Payload = serde_json::from_value(body)?;

        Ok(payload)
    }

    pub async fn write_resources(
        &self,
        commands: WriteCommands,
    ) -> Result<WriteResponse, Box<dyn std::error::Error>> {
        let res = self
            .http_client
            .get(&self.uri)
            .query(&[
                ("token", &self.token),
                ("sync_token", &"*".to_string()),
                ("commands", &json!(commands).to_string()),
            ])
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        let body = res.json::<serde_json::Value>().await?;
        let data: WriteResponse = serde_json::from_value(body)?;

        Ok(data)
    }
}

#[cfg(test)]
mod todoist_sync_api_tests {
    use super::*;

    #[test]
    fn test() {
        let client = SyncApi::new("hello".to_string());
        client.read_resources();

        assert_eq!(1, 1);
    }
}
