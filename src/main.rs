use axum::{
    extract::{Query, State, Multipart}, response::Json, routing::{get, post}, Router
};
use tokio::{
    fs,
    fs::File, 
    io::{AsyncReadExt, AsyncWriteExt}
};

use serde_derive::{Deserialize, Serialize};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{
    Client,primitives::ByteStream,presigning::PresigningConfig,
};
use std::time::Duration;

// use serde_json::json;
// use tracing_subscriber::registry::Data;
// use std::{sync::Arc, vec};
// use tokio::sync::Mutex;
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;
// use std::env;
use  chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Serialize)]
struct Notification {
    notification_int: i32,
    notification_title: String,
    product_number: i32,
}

#[derive(Deserialize)]
struct ProductId {
    product_id: i32,
}
#[derive(Deserialize)]
struct BidParams {
    product_id: i32,
    bid_price: i32,
}
#[derive(Deserialize)]
struct NotificationParams {
    notification_int: i32,
    notification_title: String,
    product_number: i32,
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct ProductionParams {
    product_id: i32,
    product_title: String,
    product_image_url: String,
    product_price: i32,
    product_openprice: i32,
    product_tags: Value,
    product_text: Value,
    created_at: Option<NaiveDateTime>,
    product_thresholds: Value,
    product_sold_status: i32,
}
#[derive(Serialize)]
struct SuccessMessage {
    status:i32,
    message: String,
}

// #[derive(Serialize)]
// struct ErrorMessage {
//     status:i32,
//     message: String,
// }
#[tokio::main]
async fn main()  {

    dotenv().ok();

    // Hello Worldと返すハンドラーを定義
    async fn root_handler() -> String {
        "Hello World".to_string()
    }
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
    .connect(&database_url)
    .await
    .expect("Failed to connect to database");

    // Router
    let app = Router::new()
        .route("/", get(root_handler))
        //.route("/users", get(get_users).post(post_user))
        // .route("/users/:user_id", patch(patch_user).delete(delete_user))
        .route("/api/v1/notification/check",get(process_notification))
        .route("/api/v1/production/create", post(create_production))
        .route("/api/v1/production/getDetail", post(get_production))
        .route("/api/v1/production/bid", post(bid_auction))
        .route("/api/v1/production/list", get(get_productions_list))
        .route("/api/v1/convenient/saveImage", post(upload_image))
        // secretは全てあとで消す
        .route("/api/v1/secret/deleteTabele", get(clear_table))
        .with_state(pool);

    // サーバの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async  fn process_notification(Query(params): Query<NotificationParams>) 
-> Result<Json<Notification>, String> {
    println!("Notification ID: {}", params.notification_int);
    println!("Notification Title: {}", params.notification_title);
    println!("Product number: {}", params.product_number);

    let notification = Notification {
        notification_int:params.notification_int,
        notification_title:params.notification_title,
        product_number:params.product_number,
    };
        Ok(Json(notification))
}

async fn clear_table(State(pool): State<PgPool>) -> Result<Json<SuccessMessage>, String> {
    sqlx::query!("DELETE FROM productions")
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(SuccessMessage{status:200,message:"Table cleared successfully".to_string()}))
}

async fn create_production(State(pool): State<PgPool>,Json(body_params): Json<ProductionParams>) -> Result<Json<SuccessMessage>, String> {

    sqlx::query!(
        "INSERT INTO productions (product_title, product_image_url, product_price, product_openprice, product_tags, product_text, product_thresholds, product_sold_status) VALUES ($1, $2, $3, $4, $5, $6, $7 ,$8)", 
        body_params.product_title,
        body_params.product_image_url,
        body_params.product_price, 
        body_params.product_openprice,
        serde_json::json!(body_params.product_tags),
        serde_json::json!(body_params.product_text),
        serde_json::json!(body_params.product_thresholds),
        0)
        .execute(&pool)
        .await
        //
        .map_err(|e| e.to_string())?;

        Ok(Json(SuccessMessage{status:200,message:"success".to_string()}))

}

#[axum::debug_handler]
async fn get_production(State(pool): State<PgPool>,Json(body_params): Json<ProductId>) -> Result<Json<ProductionParams>, String> {

    // データベースプールを作成
    let row = sqlx::query_as!(
        ProductionParams,
        r#"
        SELECT 
            product_id,
            product_title, 
            product_image_url, 
            product_price, 
            product_openprice, 
            product_tags, 
            product_text, 
            product_thresholds,
            product_sold_status,
            created_at 
        FROM productions 
        WHERE product_id = $1
        "#,
        body_params.product_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(row))
}


async fn bid_auction(State(pool): State<PgPool>,Json(body_params): Json<BidParams>) -> Result<Json<SuccessMessage>, String> {
    sqlx::query!(
        r#"
        UPDATE productions
        SET product_price = $1
        WHERE product_id = $2
        "#,
        body_params.bid_price,
        body_params.product_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(SuccessMessage{status:200,message:"success".to_string()}))
}

async fn get_productions_list(State(pool): State<PgPool>) -> Result<Json<Vec<ProductionParams>>, String> {
    let rows = sqlx::query_as!(
        ProductionParams,
        r#"
        SELECT 
            product_id,
            product_title, 
            product_image_url, 
            product_price, 
            product_openprice, 
            product_tags, 
            product_text, 
            product_thresholds,
            product_sold_status,
            created_at
        FROM productions
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;
    println!("{:?}", rows[0].product_tags);
    Ok(Json(rows))
}

async fn save_image(file_path: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let client = get_s3_client().await;
    // べつに.envじゃなくてもいいかも
    let bucket_name = std::env::var("BUCKET_NAME")?;
    let key = file_path.split('/').last().unwrap();

    let mut file = File::open(&file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let body = ByteStream::from(buffer);

    client.put_object()
    .bucket(&bucket_name)
    .key(key)
    .body(body)
    .send()
    .await?; 

    let presigned_request = client.get_object()
        .bucket(&bucket_name)
        .key(key)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(3600))?)
        .await?;

    let presigned_url = presigned_request.uri().to_string();
    println!("Presigned URL: {}", presigned_url);
    Ok(presigned_url)
}

async fn upload_image(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("file").to_string();
        let data = field.bytes().await.unwrap();
        println!("{}",name);

        // 一時ファイルとして保存してる
        let file_name = format!("/tmp/{}", name);

        if fs::metadata(&file_name).await.map(|m| m.is_dir()).unwrap_or(false) {
            return name.to_string();
        }

        let mut file = File::create(&file_name).await.unwrap();
        file.write_all(&data).await.unwrap();

        let s3_url = save_image(file_name).await.unwrap();
        return s3_url;
    }

    "No file received".to_string()
}

async fn get_s3_client() -> Client {
    // 地域の設定
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

