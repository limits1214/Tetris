use std::sync::Arc;

use crate::util;

#[derive(Debug, Clone)]
pub struct AppState {
    pub dynamo_client: aws_sdk_dynamodb::Client,
}
impl AppState {
    pub async fn new() -> Self {
        let dynamo_client = make_dynamo_client().await;
        Self { dynamo_client }
    }
}

pub struct ArcAppState(pub Arc<AppState>);
impl ArcAppState {
    pub async fn new() -> Self {
        Self(Arc::new(AppState::new().await))
    }
}
impl Clone for ArcAppState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
async fn make_dynamo_client() -> aws_sdk_dynamodb::Client {
    let shared_config = util::config::get_aws_config();
    aws_sdk_dynamodb::Client::new(shared_config)
}
