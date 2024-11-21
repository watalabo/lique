use lsp_types::notification::Notification;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcMessageRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

pub enum OutgoingMessage {
    RpcMessageResponse(RpcMessageResponse),
    NotificationMessage(NotificationMessage),
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcMessageResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub result: Value,
}

impl RpcMessageResponse {
    pub fn new<T: Serialize>(id: Option<Value>, result: T) -> anyhow::Result<Self> {
        Ok(Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: serde_json::to_value(result)?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
}

impl NotificationMessage {
    pub fn new<T: Notification>(params: <T as Notification>::Params) -> anyhow::Result<Self> {
        Ok(Self {
            jsonrpc: "2.0".to_string(),
            method: T::METHOD.to_string(),
            params: serde_json::to_value(params)?,
        })
    }
}
