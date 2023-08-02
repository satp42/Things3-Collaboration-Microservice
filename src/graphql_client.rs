use reqwest::Client;
use async_graphql::*;

pub struct GraphQLClient {
    client: Client,
    url: String,
}

impl GraphqlClient {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn get_todays_task(&self) -> Result<Vec<task::Task>, Box<dyn std::error::Error>> {
        let query = task::GetTodaysTask::build_query(task::Variables {});
        let res = self.client.post(&self.url).json(&query).send().await?;
        let body: Response<task::ResponseData> = res.json().await?;
        Ok(body.data.unwrap().tasks)
    }
}