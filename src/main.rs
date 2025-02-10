mod save_image;
mod types;

use axum::{
    extract::{Query, State}, response::Json, routing::{get, post}, Router
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;

// 別のスクリプトからインポートしてきた子達
use save_image::upload_image;
use types::{
    Notification, 
    NotificationParams, 
    ProductId, 
    BidParams, 
    ProductionParams, 
    SuccessMessage
};

#[tokio::main]
async fn main()  {

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
    .connect(&database_url)
    .await
    .expect("Failed to connect to database");

    // Router
    let app = Router::new()
        .route("/api/v1/notification/check",get(process_notification))
        .route("/api/v1/production/create", post(create_production))
        .route("/api/v1/production/getDetail", post(get_production))
        .route("/api/v1/production/bid", post(bid_auction))
        .route("/api/v1/production/list", get(get_productions_list))
        .route("/api/v1/convenient/saveImage", post(upload_image))
        // secretは全てあとで***絶対に***消す
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
        .map_err(|e| e.to_string())?;

        Ok(Json(SuccessMessage{status:200,message:"success".to_string()}))

}

#[axum::debug_handler]
async fn get_production(State(pool): State<PgPool>,Json(body_params): Json<ProductId>) -> Result<Json<ProductionParams>, String> {

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
