use crate::crypt::decryption::decrypt_data;
use crate::db::get_path::get_name_and_path_of_file;
use crate::tools::content_type::check_content_type;
use crate::{
    crypt::base64_convert::convert_base64_to_aes, file_actions::resp_errors::download_err_resp,
};

use std::convert::Infallible;
use tokio::io::AsyncReadExt;

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderName, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};

pub async fn download_file(Path(short_url): Path<String>) -> Result<impl IntoResponse, Infallible> {
    let (file_path_to_file, file_name) = match get_name_and_path_of_file(short_url).await {
        Ok((file_path, file_name, is_encrypted)) => {
            if is_encrypted == true {
                return Ok(download_err_resp(
                    String::from("Insert the AES key into URL"),
                    StatusCode::BAD_REQUEST,
                )
                .await);
            }
            (file_path, file_name)
        }
        Err(err) => {
            tracing::error!(
                %err,
                "Taking data from db is failed"
            );
            return Ok(download_err_resp(
                String::from("URL or FILE not found"),
                StatusCode::BAD_REQUEST,
            )
            .await);
        }
    };

    match tokio::fs::File::open(&file_path_to_file).await {
        Ok(mut file) => {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.unwrap();

            let len = buf.len();

            let body = axum::body::Body::from(buf);
            let mut response = Response::new(body);

            let content_type = check_content_type(&file_name);
            let content_disposition = format!("attachment; filename=\"{}\"", file_name);

            response
                .headers_mut()
                .insert(CONTENT_TYPE, content_type.parse().unwrap());
            response.headers_mut().insert(
                HeaderName::from_static("content-length"),
                HeaderValue::from_str(&len.to_string()).unwrap(),
            );
            response.headers_mut().insert(
                HeaderName::from_static("content-disposition"),
                HeaderValue::from_str(&content_disposition).unwrap(),
            );

            Ok(response)
        }
        Err(err) => {
            tracing::error!(
                %err,
                "Path to file is incorrect"
            );
            return Ok(download_err_resp(
                String::from("FILE or URL not found"),
                StatusCode::NOT_FOUND,
            )
            .await);
        }
    }
}

pub async fn download_file_with_aes(
    Path((short_url, aes_key)): Path<(String, String)>,
) -> Result<impl IntoResponse, Infallible> {
    let (file_path_to_file, file_name) = match get_name_and_path_of_file(short_url).await {
        Ok((file_path, file_name, _)) => (file_path, file_name),
        Err(err) => {
            tracing::error!(
                %err,
                "Taking data from db is failed"
            );
            return Ok(download_err_resp(
                String::from("URL or FILE not found"),
                StatusCode::BAD_REQUEST,
            )
            .await);
        }
    };

    match tokio::fs::File::open(&file_path_to_file).await {
        Ok(mut file) => {
            let key_bytes = match convert_base64_to_aes(aes_key).await {
                Ok(key) => key,
                Err(_err) => {
                    return Ok(download_err_resp(
                        String::from("Invalid key"),
                        StatusCode::NOT_FOUND,
                    )
                    .await);
                }
            };

            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.unwrap();

            let data = decrypt_data(&buf, key_bytes).await.unwrap();

            let len = data.len();

            let body = axum::body::Body::from(data);
            let mut response = Response::new(body);

            let content_type = check_content_type(&file_name);
            let content_disposition = format!("attachment; filename=\"{}\"", file_name);

            response
                .headers_mut()
                .insert(CONTENT_TYPE, content_type.parse().unwrap());
            response.headers_mut().insert(
                HeaderName::from_static("content-length"),
                HeaderValue::from_str(&len.to_string()).unwrap(),
            );
            response.headers_mut().insert(
                HeaderName::from_static("content-disposition"),
                HeaderValue::from_str(&content_disposition).unwrap(),
            );

            return Ok(response);
        }
        Err(err) => {
            tracing::error!(
                %err,
                "Path to file is incorrect"
            );
            return Ok(download_err_resp(
                String::from("FILE or URL not found"),
                StatusCode::NOT_FOUND,
            )
            .await);
        }
    }
}
