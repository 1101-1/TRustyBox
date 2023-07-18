use crate::crypt::base64_convert::convert_aes_to_base64;
use crate::crypt::{aes_key::set_aes_key, encryption::encrypt_data};
use crate::db::insert_to_db::insert_main_data;
use crate::tools::short_url::generate_short_path_url;

use futures::TryStreamExt;
use std::sync::Arc;
use std::{convert::Infallible, env};
use tokio::sync::Mutex;
use tokio::{fs::File, io::AsyncWriteExt};

use axum::{
    extract::{Multipart, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::resp_errors::upload_err_resp;
use super::types::file_type::FileFullData;
use super::types::upload_types::{UploadPayload, UploadResponse};

pub async fn upload_file(
    Query(payload): Query<UploadPayload>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Infallible> {
    let multipart = Arc::new(Mutex::new(multipart));

    if let Some(header) = payload.encryption {
        if header == "aes".to_string() || header == "aes/".to_string() {
            return Ok(upload_with_aes_ecnrypt(multipart.lock().await).await);
        }
    }

    return Ok(upload_without_ecrypt(multipart.lock().await).await);
}

async fn upload_without_ecrypt(
    mut multipart: tokio::sync::MutexGuard<'_, Multipart>,
) -> (axum::http::StatusCode, Json<UploadResponse>) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let generated_short_path = generate_short_path_url();

        let file_data = FileFullData::new(
            field.file_name().unwrap().to_owned(),
            field.headers().to_owned(),
        )
        .await;

        let mut file = match File::create(file_data.file_path.clone()).await {
            Ok(file) => file,
            Err(err) => {
                tracing::warn!(
                    %err,
                    "Cannot create file on created path"
                );
                return upload_err_resp(
                    String::from("Can't to upload file. Try again"),
                    StatusCode::BAD_REQUEST,
                )
                .await;
            }
        };

        while let Some(chunk) = field.try_next().await.unwrap() {
            file.write_all(&chunk).await.unwrap();
            file.flush().await.unwrap();
        }

        drop(file);

        if let Err(err) = insert_main_data(
            file_data.file_path,
            file_data.new_name,
            file_data.main_data.name,
            generated_short_path.clone(),
            false,
        )
        .await
        {
            tracing::warn!(
                %err,
                "Err to add info into db"
            );
            return upload_err_resp(
                String::from("Could not insert information to database. Try again"),
                StatusCode::BAD_REQUEST,
            )
            .await;
        }

        let response = UploadResponse {
            short_path: Some(generated_short_path.clone()),
            full_url: Some(format!(
                "http://{}/{}",
                env::var("SERVER_ADDR").expect("ADDR NOT FOUND"),
                &generated_short_path,
            )),
            error: None,
            aes_key: None,
        };
        return (StatusCode::OK, Json(response));
    }
    upload_err_resp(
        String::from("FILE to download NOT FOUND"),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
    .await
}

async fn upload_with_aes_ecnrypt(
    mut multipart: tokio::sync::MutexGuard<'_, Multipart>,
) -> (axum::http::StatusCode, Json<UploadResponse>) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let file_data = FileFullData::new(
            field.file_name().unwrap().to_owned(),
            field.headers().to_owned(),
        )
        .await;

        let generated_short_path = generate_short_path_url();

        let mut file = match File::create(file_data.file_path.clone()).await {
            Ok(file) => file,
            Err(err) => {
                tracing::warn!(
                    %err,
                    "Cannot create file on created path"
                );
                return upload_err_resp(
                    String::from("Could not encrypt data. Try again."),
                    StatusCode::BAD_REQUEST,
                )
                .await;
            }
        };

        let mut data = Vec::new();
        while let Some(chunk) = field.try_next().await.unwrap() {
            data.extend_from_slice(&chunk);
        }

        let aes_key = set_aes_key().await;
        let encoded_key = convert_aes_to_base64(aes_key).await;

        let encrypted_data = match encrypt_data(&data, aes_key).await {
            Ok(encrypted_data) => encrypted_data,
            Err(err) => {
                tracing::error!(
                    %err,
                    "Encryption is failed"
                );
                return upload_err_resp(
                    String::from("Could not encrypt data. Try again."),
                    StatusCode::BAD_REQUEST,
                )
                .await;
            }
        };

        file.write_all(&encrypted_data).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        if let Err(err) = insert_main_data(
            file_data.file_path.clone(),
            file_data.new_name,
            file_data.main_data.name,
            generated_short_path.clone(),
            false,
        )
        .await
        {
            tracing::warn!(
                %err,
                "Err to add info into db"
            );
            return upload_err_resp(
                String::from("Could not insert information to database. Try again"),
                StatusCode::BAD_REQUEST,
            )
            .await;
        }

        let response = UploadResponse {
            short_path: Some(generated_short_path.clone()),
            full_url: Some(format!(
                "http://{}/{}/{}",
                env::var("SERVER_ADDR").expect("ADDR NOT FOUND"),
                &generated_short_path,
                encoded_key,
            )),
            error: None,
            aes_key: Some(encoded_key),
        };
        return (StatusCode::OK, Json(response));
    }

    upload_err_resp(
        String::from("FILE to download NOT FOUND"),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
    .await
}
