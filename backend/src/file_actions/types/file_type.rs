use std::env;

use axum::http::HeaderMap;

use crate::tools::generate_uuid::generate_uuid_v4;

#[derive(Clone)]
pub struct FileMainData {
    pub name: String,
    pub headers: HeaderMap,
}

#[derive(Clone)]
pub struct FileFullData {
    pub main_data: FileMainData,
    pub len: usize,
    pub new_name: String,
    pub file_path: String,
}

impl FileMainData {
    pub async fn new(name: String, headers: HeaderMap) -> Self {
        Self { name, headers }
    }
}

impl FileFullData {
    pub async fn new(name: String, headers: HeaderMap) -> Self {
        let main_data = FileMainData::new(name, headers).await;

        let new_name = FileFullData::set_new_name(main_data.clone()).await;

        Self {
            len: FileFullData::set_len(main_data.clone()).await,
            new_name: new_name.clone(),
            file_path: FileFullData::set_path(new_name).await,
            main_data,
        }
    }

    pub async fn set_len(main_data: FileMainData) -> usize {
        let file_len = if let Some(len) = main_data
            .headers
            .get("Content-Length")
            .and_then(|header| Some(header.len()))
        {
            len
        } else {
            0usize
        };

        file_len
    }

    pub async fn set_path(new_name: String) -> String {
        let file_path = format!(
            "{}{}",
            env::var("PATH_TO_FILES").expect("Unexpected error"),
            new_name
        );

        file_path
    }

    pub async fn set_new_name(main_data: FileMainData) -> String {
        let new_filename = match main_data.name.split('.').last() {
            Some(extension) => format!("{}.{}", generate_uuid_v4().await, extension),
            None => generate_uuid_v4().await,
        };

        new_filename
    }
}
