mod save_image;
mod types;
mod auth;

use axum::{
    extract::{Query, State,},
    response::Json,
    routing::{get, post},
    Router,
};
use axum_extra::headers::{Authorization, authorization::Bearer};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::sync::Arc;
use dashmap::DashMap;
use axum_extra::TypedHeader;
use tower_http::cors::{CorsLayer, Any};

// インポートしてきた型
use types::types::{
    Notification,
    NotificationParams, 
    ProductId, 
    BidParams,
    ProductionParams, 
    SuccessMessage,
    TokenStore,
    AppState,
};

// インポートしてきた関数
use save_image::upload_image;
use auth::auth::login;

#[tokio::main]
async fn main()  {

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let token_store: TokenStore = Arc::new(DashMap::new());

    let state = Arc::new(AppState {  
        pool,
        token_store: token_store.clone(),
    });

    // CORS設定
    let cors = CorsLayer::new()
    .allow_origin(Any)  // 必要に応じて特定のオリジンに制限することができます
    .allow_methods(Any)
    .allow_headers(Any);

    // Router
    let app = Router::new()
        .route("/api/v1/notification/check", get(process_notification))
        .route("/api/v1/production/create", post(create_production))
        .route("/api/v1/production/getDetail", post(get_production))
        .route("/api/v1/production/bid", post(bid_auction))
        .route("/api/v1/production/list", get(get_productions_list))
        .route("/api/v1/convenient/saveImage", post(upload_image))
        .route("/api/v1/auth/login", get({
            let token_store = token_store.clone();
            move || login(State(token_store))
        }))
        .route("/api/v1/secret/deleteTabele", get(clear_table))
        // ここでstateを渡している
        // ここでstateを渡せないときは多分各APIでstateを受け取らないようになっている。
        .layer(cors)
        .with_state(state.clone());

    // サーバの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// あとでnotification.rsを作って移動する。
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

// これはリリース前に絶対消す。
async fn clear_table(State(state): State<Arc<AppState>>) -> Result<Json<SuccessMessage>, String> {
    sqlx::query!("DELETE FROM productions")
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(SuccessMessage{status:200,message:"Table cleared successfully".to_string()}))
}

// オークション作成API
async fn create_production(
    State(state): State<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(body_params): Json<ProductionParams>
) -> Result<Json<SuccessMessage>, String> {

    // let token = bearer.token();

    // let user_id = "user123"; // 実際にはトークンからユーザーIDを抽出する必要があります
    // if verify_token(token_store.clone(), user_id.to_string(), token.to_string()).is_none() {
    //     return Err("Invalid token".to_string());
    // }

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
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Json(SuccessMessage{status:200,message:"success".to_string()}))

}

#[axum::debug_handler]
async fn get_production(State(state): State<Arc<AppState>>,Json(body_params): Json<ProductId>) -> Result<Json<ProductionParams>, String> {

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
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(row))
}


async fn bid_auction(State(state): State<Arc<AppState>>,Json(body_params): Json<BidParams>) -> Result<Json<SuccessMessage>, String> {
    sqlx::query!(
        r#"
        UPDATE productions
        SET product_price = $1
        WHERE product_id = $2
        "#,
        body_params.bid_price,
        body_params.product_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(SuccessMessage{status:200,message:"success".to_string()}))
}

// オークション一覧取得API
// もっとユーザーの興味に合わせたリストにする
async fn get_productions_list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<ProductionParams>>, String> {
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
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    println!("{:?}", rows[0].product_tags);
    Ok(Json(rows))
}
