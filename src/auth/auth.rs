// mod.rsを使ってインポートした際は<モジュール名>::<モジュール名>みたいになるっぽい。
use crate::types::types::{
    Claims,
    TokenStore,
    LoginResult,
    Token,
};
use axum::{
    extract::State, response::Json
};

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

// あとで.envに
const SECRET_KEY: &[u8] = b"";

// ログイン
// todo: 現在は固定のユーザーIDを使っているが、実際はDBなどから取得する
pub async fn login(token_store: State<TokenStore>) -> Result<Json<LoginResult>, String> {
    let token = generate_token("user123");
    token_store.insert("user123".to_string(), token.clone());

    Ok(Json(LoginResult {
        id: "user123".to_string(),
        enable_time: 300,
        token_data: Token { 
            token
        },
    }))
    //format!("Bearer {}", token)
}

// トークンを発行する関数
fn generate_token(user_id: &str) -> String {
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 300; // 5分有効

    let claims = Claims {
        id: user_id.to_string(),
        enable_time: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).expect("Failed to generate token")
}

// トークンを検証して、新しいトークンを発行する
// リフレッシュトークンとしてAPI化する？？
pub fn verify_and_refresh_token(token: &str, token_store: &TokenStore) -> Option<String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ).ok()?;

    let user_id = token_data.claims.id;

    // 古いトークンが登録されているか確認
    if let Some(existing_token) = token_store.get(&user_id) {
        if existing_token.value() != token {
            return None; // トークンが一致しない場合、不正とみなす
        }
    }

    // 新しいトークンを発行
    let new_token = generate_token(&user_id);

    // トークンを更新
    token_store.insert(user_id.clone(), new_token.clone());

    Some(new_token)
}
