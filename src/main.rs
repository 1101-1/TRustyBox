use dotenv::dotenv;

use axum::{
    extract::{DefaultBodyLimit, Path},
    http::{header::CONTENT_TYPE, HeaderName, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post, Router},
    Json,
};
use axum_typed_multipart::{FieldData, TempFile, TryFromMultipart, TypedMultipart};
use serde::Serialize;
use std::{convert::Infallible, env};
use tokio::io::AsyncReadExt;

mod connection_to_db;
mod tools;

async fn download_file(Path(short_url): Path<String>) -> Result<impl IntoResponse, Infallible> {
    let (file_path_to_file, file_name, changed_name) =
        match connection_to_db::get_name_and_path_of_file(short_url).await {
            Ok((file_path, file_name, uuid_name)) => (file_path, file_name, uuid_name),
            Err(_err) => {
                let response = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("URL or FILE not found".into())
                    .unwrap();
                return Ok(response);
            }
        };

    let _ = &file_path_to_file.replace(&changed_name, &file_name);
    match tokio::fs::File::open(&file_path_to_file).await {
        Ok(mut file) => {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.unwrap();

            let len = buf.len();

            let body = axum::body::Body::from(buf);
            let mut response = Response::new(body);

            let content_type = tools::check_content_type(&file_name).await;
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
        Err(_) => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("FILE or URL not found".into())
                .unwrap();
            Ok(response)
        }
    }
}

async fn upload_file(
    TypedMultipart(RequestData { file }): TypedMultipart<RequestData>,
) -> Result<impl IntoResponse, Infallible> {
    let file_name = file.metadata.file_name.unwrap_or("data.bin".to_string());
    let new_filename = match file_name.split('.').last() {
        Some(extension) => format!("{}.{}", tools::generate_uuid_v4().await, extension),
        None => tools::generate_uuid_v4().await,
    };

    let path = format!(
        "{}{}",
        env::var("PATH_TO_FILES").expect("Unexpected error"),
        new_filename
    );

    let generated_short_path = tools::generate_short_path_url().await;

    match file.contents.persist(&path, false).await {
        Ok(_) => {
            match connection_to_db::insert_to_mongodb(
                &path,
                &new_filename,
                &file_name,
                generated_short_path.clone(),
            )
            .await
            {
                Ok(()) => (),
                Err(_err) => {
                    let response = UploadResponse {
                        short_path: None,
                        url: None,
                        error: Some("Could not insert information to database".to_string()),
                    };
                    return Ok((StatusCode::BAD_REQUEST, Json(response)));
                }
            };
        }
        Err(_err) => {
            let response = UploadResponse {
                short_path: None,
                url: None,
                error: Some("Can not encrypt the file. Try again".to_string()),
            };
            return Ok((StatusCode::OK, Json(response)));
        }
    };
    let response = UploadResponse {
        short_path: Some(generated_short_path.clone()),
        url: Some(format!(
            "http://{}/{}",
            env::var("SERVER_ADDR").expect("ADDR NOT FOUND"),
            &generated_short_path
        )),
        error: None,
    };
    return Ok((StatusCode::OK, Json(response)));
}

#[derive(Serialize)]
struct UploadResponse {
    short_path: Option<String>,
    error: Option<String>,
    url: Option<String>,
}

#[derive(TryFromMultipart)]
struct RequestData {
    file: FieldData<TempFile>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", post(upload_file))
        .route("/:path", get(download_file))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));

    let addr = env::var("SERVER_ADDR")
        .expect("ADDR NOT FOUND")
        .parse()
        .unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
