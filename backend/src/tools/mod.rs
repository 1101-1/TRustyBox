pub mod content_type;
pub mod http_tools;
pub mod short_url;

pub async fn generate_uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}
