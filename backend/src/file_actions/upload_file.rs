use crate::crypt::base64_convert::convert_aes_to_base64;
use crate::crypt::{aes_key::set_aes_key, encryption::encrypt_data};
use crate::db::insert_to_mongo::insert_to_mongodb;
use crate::tools::generate_short_path_url::generate_short_path_url;
use crate::tools::generate_uuid::generate_uuid_v4;

use futures::TryStreamExt;
use std::{convert::Infallible, env};
use tokio::{fs::File, io::AsyncWriteExt};

use axum::{
    extract::{Multipart, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::types::upload_types::{UploadPayload, UploadResponse};

pub async fn upload_file(
    Query(payload): Query<UploadPayload>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, Infallible> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_owned();
        let new_filename = match file_name.split('.').last() {
            Some(extension) => format!("{}.{}", generate_uuid_v4().await, extension),
            None => generate_uuid_v4().await,
        };

        let generated_short_path = generate_short_path_url().await;

        let file_path = format!(
            "{}{}",
            env::var("PATH_TO_FILES").expect("Unexpected error"),
            new_filename
        );

        let mut file = match File::create(&file_path).await {
            Ok(file) => file,
            Err(_err) => {
                let response = UploadResponse {
                    short_path: None,
                    full_url: None,
                    error: Some("Can't to upload file. Try again".to_string()),
                    aes_key: None,
                };
                return Ok((StatusCode::BAD_REQUEST, Json(response)));
            }
        };
        if let Some(header) = payload.encryption {
            if header == "aes".to_string() || header == "aes/".to_string() {
                let mut data = Vec::new();
                while let Some(chunk) = field.try_next().await.unwrap() {
                    data.extend_from_slice(&chunk);
                }

                let aes_key = set_aes_key().await;
                let encoded_key = convert_aes_to_base64(aes_key).await;

                let encrypted_data = match encrypt_data(&data, aes_key).await {
                    Ok(encrypted_data) => encrypted_data,
                    Err(_err) => {
                        let response = UploadResponse {
                            short_path: None,
                            full_url: None,
                            error: Some("Could not encrypt data. Try again.".to_string()),
                            aes_key: None,
                        };
                        return Ok((StatusCode::BAD_REQUEST, Json(response)));
                    }
                };
                file.write_all(&encrypted_data).await.unwrap();
                file.flush().await.unwrap();
                drop(file);
                match insert_to_mongodb(
                    &file_path,
                    &new_filename,
                    &file_name,
                    generated_short_path.clone(),
                    true,
                )
                .await
                {
                    Ok(()) => (),
                    Err(_err) => {
                        let response = UploadResponse {
                            short_path: None,
                            full_url: None,
                            error: Some(
                                "Could not insert information to database. Try again".to_string(),
                            ),
                            aes_key: None,
                        };
                        return Ok((StatusCode::BAD_REQUEST, Json(response)));
                    }
                };

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
                return Ok((StatusCode::OK, Json(response)));
            }
        }

        while let Some(chunk) = field.try_next().await.unwrap() {
            file.write_all(&chunk).await.unwrap();
            file.flush().await.unwrap();
        }

        drop(file);
        match insert_to_mongodb(
            &file_path,
            &new_filename,
            &file_name,
            generated_short_path.clone(),
            false,
        )
        .await
        {
            Ok(()) => (),
            Err(_err) => {
                let response = UploadResponse {
                    short_path: None,
                    full_url: None,
                    error: Some("Could not insert information to database. Try again".to_string()),
                    aes_key: None,
                };
                return Ok((StatusCode::BAD_REQUEST, Json(response)));
            }
        };

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
        return Ok((StatusCode::OK, Json(response)));
    }
    let response = UploadResponse {
        short_path: None,
        full_url: None,
        error: Some("FILE to download NOT FOUND".to_string()),
        aes_key: None,
    };
    return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(response)));
}
