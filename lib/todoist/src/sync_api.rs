pub struct SyncApi {
    token: String,
    uri: String,
}

impl SyncApi {
    pub fn new(token: String) -> Self {
        Self {
            token,
            uri: format!("{}/sync", super::todoist::BASE_URI),
        }
    }

    pub async fn read_resources(self) -> Result<serde_json::Value, reqwest::Error>  {
            let client = reqwest::Client::new();

            let res = client
                .get(self.uri)
                .query(&[("token", self.token), ("sync_token", "*".to_string()), ("resource_types", "[\"all\"]".to_string())])
                .header(reqwest::header::ACCEPT, "application/json")
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

        Ok(res)
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
