// use std::fs::read_to_string;
// use serde_derive::{Deserialize, Serialize};
// use std::sync::Arc;
// use tokio::sync::Mutex;
// use axum::{response::Html, Router, routing::get};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use axum::{routing::get, Router};

// #[tokio::main]  // main関数を非同期関数にするために必要
// async fn main() {
//     // ロギングの初期化(ログが出力されるようにしている)
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "rustwi=debug,tower_http=debug".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     let app:Router = Router::new().route("/", get(handler));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();
//     tracing::debug!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

// async fn handler() -> Html<&'static str> {
//     Html("<h1>Hello, World!</h1>")
// }

// #[tokio::main]
// async fn main() {
//     async fn root_handler() -> String {
//         "Hello world".read_to_string()
//     }

//     let app = Router::new().route("/",get(root_handler));

//     let listener = tokio::new::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// #[tokio::main]
// async fn main() {
//     #[derive(Clone, Deserialize, Serialize)]
//     struct User {
//         id: u32,
//         name: String,
//     }

//     #[derive(Clone, Deserialize, Serialize)]
//     struct Users {
//         users: Vec<User>,
//     }

//     let users = Users {
//         users: vec![
//             User {
//                 id: 1,
//                 name: "takashi".to_string(),
//             },
//             User {
//                 id: 2,
//                 name: "hitoshi".to_string(),
//             },
//             User {
//                 id: 3,
//                 name: "masashi".to_string(),
//             },
//         ],
//     };

//     let users_state = Arc::new(Mutex::new(users));

//     // Hello Worldと返すハンドラーを定義
//     async fn root_handler() -> String {
//         "Hello World".to_string()
//     }

//     // Read
//     async fn get_users(State(users_state): State<Arc<Mutex<Users>>>) -> Json<Users> {
//         let user_lock = users_state.lock().await;
//         Json(user_lock.clone())
//     }

//     #[derive(Clone, Serialize, Deserialize)]
//     struct CreateUser {
//         name: String,
//     }

//     // Create
//     async fn post_user(
//         State(users_state): State<Arc<Mutex<Users>>>,
//         create_user: Json<CreateUser>,
//     ) -> Json<Users> {
//         let mut users_lock = users_state.lock().await;

//         let new_user = User {
//             id: (users_lock.users.len() + 1) as u32,
//             name: create_user.name.to_string(),
//         };

//         users_lock.users.push(new_user);

//         Json(users_lock.clone())
//     }

//     // Update
//     async fn patch_user(
//         State(users_state): State<Arc<Mutex<Users>>>,
//         Path(user_id): Path<u32>,
//         Json(update_user): Json<CreateUser>,
//     ) -> Result<Json<User>, String> {
//         let mut users_lock = users_state.lock().await;

//         if let Some(user) = users_lock.users.iter_mut().find(|user| user.id == user_id) {
//             user.name = update_user.name.clone();
//             return Ok(Json(user.clone()));
//         }

//         Err("User not found".to_string())
//     }

//     // Delete
//     async fn delete_user(
//         State(users_state): State<Arc<Mutex<Users>>>,
//         Path(user_id): Path<u32>,
//     ) -> Result<Json<Users>, String> {
//         let mut users_lock = users_state.lock().await;

//         // 更新前のusersの長さを保持
//         let original_len = users_lock.users.len();

//         // retainを使って、指定したIDのユーザーを削除
//         users_lock.users.retain(|user| user.id != user_id);

//         // usersの長さが変わっていれば、削除に成功している
//         if users_lock.users.len() == original_len {
//             Err("User not found".to_string())
//         } else {
//             Ok(Json(users_lock.clone()))
//         }
//     }

//     // Router
//     let app = Router::new()
//         .route("/", get(root_handler))
//         .route("/users", get(get_users).post(post_user))
//         .route("/users/:user_id", patch(patch_user).delete(delete_user))
//         .with_state(users_state);

//     // サーバの起動
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// } 


use axum::{
    extract::{Path, State},
    extract::{Query},
    response::Json,
    routing::{get, patch},
    Router,
};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize)]
struct Notification {
    notification_int: i32,
    notification_title: String,
    product_number: i32,
}

#[derive(Deserialize)]
struct NotificationParams {
    notification_int: i32,
    notification_title: String,
    product_number: i32,
}

#[tokio::main]
async fn main() {
    #[derive(Clone, Deserialize, Serialize)]
    struct User {
        id: u32,
        name: String,
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct Users {
        users: Vec<User>,
    }

    // let notification_int = 101;
    // let notification_title = String::from("Product Update");
    // let product_number = 42;

    

    let users = Users {
        users: vec![
            User {
                id: 1,
                name: "takashi".to_string(),
            },
            User {
                id: 2,
                name: "hitoshi".to_string(),
            },
            User {
                id: 3,
                name: "masashi".to_string(),
            },
        ],
    };

    let users_state = Arc::new(Mutex::new(users));

    // Hello Worldと返すハンドラーを定義
    async fn root_handler() -> String {
        "Hello World".to_string()
    }

    // Read
    async fn get_users(State(users_state): State<Arc<Mutex<Users>>>) -> Json<Users> {
        let user_lock = users_state.lock().await;
        Json(user_lock.clone())
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct CreateUser {
        name: String,
    }

    // Create
    async fn post_user(
        State(users_state): State<Arc<Mutex<Users>>>,
        create_user: Json<CreateUser>,
    ) -> Json<Users> {
        let mut users_lock = users_state.lock().await;

        let new_user = User {
            id: (users_lock.users.len() + 1) as u32,
            name: create_user.name.to_string(),
        };

        users_lock.users.push(new_user);

        Json(users_lock.clone())
    }

    // Update
    async fn patch_user(
        State(users_state): State<Arc<Mutex<Users>>>,
        Path(user_id): Path<u32>,
        Json(update_user): Json<CreateUser>,
    ) -> Result<Json<User>, String> {
        let mut users_lock = users_state.lock().await;

        if let Some(user) = users_lock.users.iter_mut().find(|user| user.id == user_id) {
            user.name = update_user.name.clone();
            return Ok(Json(user.clone()));
        }

        Err("User not found".to_string())
    }

    // Delete
    async fn delete_user(
        State(users_state): State<Arc<Mutex<Users>>>,
        Path(user_id): Path<u32>,
    ) -> Result<Json<Users>, String> {
        let mut users_lock = users_state.lock().await;

        // 更新前のusersの長さを保持
        let original_len = users_lock.users.len();

        // retainを使って、指定したIDのユーザーを削除
        users_lock.users.retain(|user| user.id != user_id);

        // usersの長さが変わっていれば、削除に成功している
        if users_lock.users.len() == original_len {
            Err("User not found".to_string())
        } else {
            Ok(Json(users_lock.clone()))
        }
    }

    // Router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", get(get_users).post(post_user))
        .route("/users/:user_id", patch(patch_user).delete(delete_user))
        .route("/notification/check",get(process_notification))
        .with_state(users_state);

    println!("vvv");

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


//i32型の引数notificationIntとstring型の引数notificationTitleとi32型の引数productNumberをとる関数
//以上を加工してJsonにする
//返却されるJsonは以下
// {
//     notificationInt:i32,
//     notificationTitle:string,
//     productNumber:i32
// }
