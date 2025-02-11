use axum::extract::Multipart;
use tokio::{
    fs,
    fs::File, 
    io::{AsyncReadExt, AsyncWriteExt}
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{
    Client,primitives::ByteStream,presigning::PresigningConfig,
};
use std::time::Duration;
use dotenvy::dotenv;

pub async fn upload_image(mut multipart: Multipart) -> String {
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

    // エラーハンドリングする
    // あとで
    "ファイルが違うぜ！".to_string()
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

async fn get_s3_client() -> Client {
    // 地域の設定
    // 特に地域以外は設定する必要はないっぽい
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}
