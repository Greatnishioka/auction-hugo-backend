use  chrono::NaiveDateTime;
use serde_json::Value;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use dashmap::DashMap;

// トークンを保存するためのキャッシュ
pub type TokenStore = Arc<DashMap<String, String>>;

#[derive(Serialize)]
pub struct Notification {
    pub notification_int: i32,
    pub notification_title: String,
    pub product_number: i32,
}

#[derive(Deserialize)]
pub struct ProductId {
    pub product_id: i32,
}
#[derive(Deserialize)]
pub struct BidParams {
    pub product_id: i32,
    pub bid_price: i32,
}
#[derive(Deserialize)]
pub struct NotificationParams {
    pub notification_int: i32,
    pub notification_title: String,
    pub product_number: i32,
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ProductionParams {
    pub product_id: i32,
    pub product_title: String,
    pub product_image_url: String,
    pub product_price: i32,
    pub product_openprice: i32,
    pub product_tags: Value,
    pub product_text: Value,
    pub created_at: Option<NaiveDateTime>,
    pub product_thresholds: Value,
    pub product_sold_status: i32,
}
#[derive(Serialize)]
pub struct SuccessMessage {
    pub status:i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub enable_time: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub id: String,
    pub enable_time: usize,
    pub token_data: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token{
    pub token: String,
}
