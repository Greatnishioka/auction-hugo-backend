use axum::{
    // extract::{Path, State},
    extract::{Query},
    response::Json,
    routing::{get, post},
    Router,
};
use serde_derive::{Deserialize, Serialize};
// use serde_json::json;
// use tracing_subscriber::registry::Data;
// use std::{sync::Arc, vec};
// use tokio::sync::Mutex;
use sqlx::PgPool;
// use dotenvy::dotenv;
use std::env;
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
struct NotificationParams {
    notification_int: i32,
    notification_title: String,
    product_number: i32,
}
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
// #[derive(Deserialize)]
// struct product_tags {
//     tags: Vec<String>,
// }
// #[derive(Deserialize)]
// struct Threshold {
//     amount: i32,
//     image_url: String,
//     title: String,
//     description: String,
//     status: String,
// }

// struct product_thresholds {
//     product_thresholds:[[50000,"https://pbs.twimg.com/media/DKaZg4YVYAAUB9B?format=jpg&name=large","椎名林檎直筆お手紙","椎名林檎本人が落札者様のお名前入りの直筆の感謝の手紙を作成して、ご自宅に送付させていただきます。","0"],[85000,"https://img.ips.co.jp/rm/02/3102417125/3102417125-x1000.jpg","椎名林檎サイン入りCD","椎名林檎がこれまでに制作したCD/アナログ盤(*1)の中から、お好みのものに椎名林檎本人がサインを直筆でいれてお送りさせていただきます。1.オリジナルのみ。他アーティストとのコラボCDや、カバー曲のみを扱ったCDなどは対象外。また、現在、有限会社黒猫堂事務所内に在庫が存在するものに限ります。(在庫リストは落札後5営業日以内にご登録メールアドレスに送付させていただきます。)","1"],[100000,"https://afflux.jp/wp-content/uploads/2020/05/AdobeStock_255741803-1024x682.jpeg","椎名林檎ビデオレター","椎名林檎本人が出演する30秒程度のビデオ(無編集)を落札者様のために作成します。また、ビデオ内で答えられる簡単な内容のものであれば、事前に質問をお送りいただけると椎名林檎本人から回答が返ってくる場合があります(*1)。1.確約はできません。また、質問の内容は椎名林檎のイメージを損なうことのないものに限定しております。","1"]]
// }


#[tokio::main]
async fn main()  {
    // #[derive(Clone, Deserialize, Serialize)]
    // struct User {
    //     id: u32,
    //     name: String,
    // }

    // #[derive(Clone, Deserialize, Serialize)]
    // struct Users {
    //     users: Vec<User>,
    // }

    // dotenv().ok();

    // let users = Users {
    //     users: vec![
    //         User {
    //             id: 1,
    //             name: "takashi".to_string(),
    //         },
    //         User {
    //             id: 2,
    //             name: "hitoshi".to_string(),
    //         },
    //         User {
    //             id: 3,
    //             name: "masashi".to_string(),
    //         },
    //     ],
    // };

    // let users_state = Arc::new(Mutex::new(users));

    // Hello Worldと返すハンドラーを定義
    async fn root_handler() -> String {
        "Hello World".to_string()
    }

    // Router
    let app = Router::new()
        .route("/", get(root_handler))
//.route("/users", get(get_users).post(post_user))
        // .route("/users/:user_id", patch(patch_user).delete(delete_user))
        .route("/notification/check",get(process_notification))
        .route("/production/create",get(create_production().await.unwrap()))
        .route("/production/getDetail", post(get_production));
        // .with_state(users_state);

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

//今回はAPI化はしない！
async fn create_production()-> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _pool = PgPool::connect(&database_url).await?;

    sqlx::query!(
        "INSERT INTO productions (product_title, product_image_url, product_price, product_openprice, product_tags, product_text, product_thresholds, product_sold_status) VALUES ($1, $2, $3, $4, $5, $6, $7 ,$8)", 
        "仮タイトル",
        "https://hochi.news/images/2024/06/17/20240617-OHT1I51107-L.jpg",
        1000, 
        1000, 
        serde_json::json!({"tags":["ticket","test椎名林檎"]}),
        serde_json::json!({"product_text":["aaa","bbb"]}),
        serde_json::json!({"product_thresholds":[[50000,"https://pbs.twimg.com/media/DKaZg4YVYAAUB9B?format=jpg&name=large","椎名林檎直筆お手紙","椎名林檎本人が落札者様のお名前入りの直筆の感謝の手紙を作成して、ご自宅に送付させていただきます。","0"],[85000,"https://img.ips.co.jp/rm/02/3102417125/3102417125-x1000.jpg","椎名林檎サイン入りCD","椎名林檎がこれまでに制作したCD/アナログ盤(*1)の中から、お好みのものに椎名林檎本人がサインを直筆でいれてお送りさせていただきます。1.オリジナルのみ。他アーティストとのコラボCDや、カバー曲のみを扱ったCDなどは対象外。また、現在、有限会社黒猫堂事務所内に在庫が存在するものに限ります。(在庫リストは落札後5営業日以内にご登録メールアドレスに送付させていただきます。)","1"],[100000,"https://afflux.jp/wp-content/uploads/2020/05/AdobeStock_255741803-1024x682.jpeg","椎名林檎ビデオレター","椎名林檎本人が出演する30秒程度のビデオ(無編集)を落札者様のために作成します。また、ビデオ内で答えられる簡単な内容のものであれば、事前に質問をお送りいただけると椎名林檎本人から回答が返ってくる場合があります(*1)。1.確約はできません。また、質問の内容は椎名林檎のイメージを損なうことのないものに限定しております。","1"]]}),
        0)
        .execute(&_pool)
        .await?;

    Ok(())
}

#[axum::debug_handler]
async fn get_production(Json(body_params): Json<ProductId>) -> Result<Json<ProductionParams>, String> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // データベースプールを作成
    let pool = PgPool::connect(&database_url).await.map_err(|e| e.to_string())?;

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

    println!("product_title: {}", row.product_title);
    println!("product_image_url: {}", row.product_image_url);
    println!("product_price: {}", row.product_price);
    println!("product_openprice: {}", row.product_openprice);
    println!("product_tags: {}", row.product_tags);
    println!("product_text: {}", row.product_text);
    println!("product_thresholds: {}", row.product_thresholds);
    println!("product_sold_status: {}", row.product_sold_status);

    Ok(Json(row))
}

// async fn get_users(State(users_state): State<Arc<Mutex<Users>>>) -> Json<Users> {
//     let user_lock = users_state.lock().await;
//     Json(user_lock.clone())
// }

// #[derive(Clone, Serialize, Deserialize)]
// struct CreateUser {
//     name: String,
// }

// // Create
// async fn post_user(
//     State(users_state): State<Arc<Mutex<Users>>>,
//     create_user: Json<CreateUser>,
// ) -> Json<Users> {
//     let mut users_lock = users_state.lock().await;

//     let new_user = User {
//         id: (users_lock.users.len() + 1) as u32,
//         name: create_user.name.to_string(),
//     };

//     users_lock.users.push(new_user);

//     Json(users_lock.clone())
// }

// // Update
// async fn patch_user(
//     State(users_state): State<Arc<Mutex<Users>>>,
//     Path(user_id): Path<u32>,
//     Json(update_user): Json<CreateUser>,
// ) -> Result<Json<User>, String> {
//     let mut users_lock = users_state.lock().await;

//     if let Some(user) = users_lock.users.iter_mut().find(|user| user.id == user_id) {
//         user.name = update_user.name.clone();
//         return Ok(Json(user.clone()));
//     }

//     Err("User not found".to_string())
// }

// // Delete
// async fn delete_user(
//     State(users_state): State<Arc<Mutex<Users>>>,
//     Path(user_id): Path<u32>,
// ) -> Result<Json<Users>, String> {
//     let mut users_lock = users_state.lock().await;

//     // 更新前のusersの長さを保持
//     let original_len = users_lock.users.len();

//     // retainを使って、指定したIDのユーザーを削除
//     users_lock.users.retain(|user| user.id != user_id);

//     // usersの長さが変わっていれば、削除に成功している
//     if users_lock.users.len() == original_len {
//         Err("User not found".to_string())
//     } else {
//         Ok(Json(users_lock.clone()))
//     }
// }

