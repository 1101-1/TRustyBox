use axum::http::{HeaderMap, HeaderValue};

#[derive(Clone)]
pub struct FileData {
    pub name: String,
    pub new_name: String,
    pub file_path: String,
    pub headers: HeaderMap,
    pub len: usize,
}

impl FileData {
    pub async fn new(
        name: String,
        new_name: String,
        file_path: String,
        headers: HeaderMap,
    ) -> Self {
        let file_len = if let Some(len) = headers
            .get("Content-Length")
            .and_then(|header| Some(header.len()))
        {
            len
        } else {
            0usize
        };

        Self {
            name,
            new_name,
            file_path,
            headers,
            len: file_len,
        }
    }

    pub async fn _get_header_val(&self, header: &str) -> Option<&HeaderValue> {
        self.headers.get(header)
    }

    pub async fn _get_all_headers(&self) -> HeaderMap {
        self.headers.clone()
    }
}
