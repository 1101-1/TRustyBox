use async_recursion::async_recursion;
use rand::Rng;

// use crate::connection_to_db::check_short_url_mongodb;

pub async fn generate_uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub async fn check_content_type(filename: &String) -> &'static str {
    match filename.split('.').last() {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("xml") => "application/xml",
        Some("pdf") => "application/pdf",
        Some("zip") => "application/zip",
        Some("tar") => "application/x-tar",
        Some("gz") => "application/gzip",
        Some("bz2") => "application/x-bzip2",
        Some("7z") => "application/x-7z-compressed",
        Some("rar") => "application/x-rar-compressed",
        Some("txt") => "text/plain",
        Some("rtf") => "application/rtf",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("xls") => "application/vnd.ms-excel",
        Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        Some("ppt") => "application/vnd.ms-powerpoint",
        Some("pptx") => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        Some("csv") => "text/csv",
        Some("tsv") => "text/tab-separated-values",
        Some("mp3") => "audio/mpeg",
        Some("wav") => "audio/wav",
        Some("aac") => "audio/aac",
        Some("flac") => "audio/flac",
        Some("opus") => "audio/opus",
        Some("ogg") => "audio/ogg",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("avi") => "video/x-msvideo",
        Some("mkv") => "video/x-matroska",
        Some("mov") => "video/quicktime",
        Some("wmv") => "video/x-ms-wmv",
        Some("gif") => "image/gif",
        Some("jpg") => "image/jpeg",
        Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("bmp") => "image/bmp",
        Some("ico") => "image/x-icon",
        Some("svg") => "image/svg+xml",
        Some("tiff") => "image/tiff",
        Some("psd") => "image/vnd.adobe.photoshop",
        Some("ai") => "application/postscript",
        Some("eps") => "application/postscript",
        Some("mpg") => "video/mpeg",
        Some("mpeg") => "video/mpeg",
        Some("weba") => "audio/webm",
        Some("docm") => "application/vnd.ms-word.document.macroEnabled.12",
        Some("dotx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
        Some("dotm") => "application/vnd.ms-word.template.macroEnabled.12",
        Some("xlsm") => "application/vnd.ms-excel.sheet.macroEnabled.12",
        Some("xltx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
        Some("xltm") => "application/vnd.ms-excel.template.macroEnabled.12",
        Some("exe") => "application/x-msdownload",
        Some("msi") => "application/x-msdownload",
        Some("dll") => "application/x-msdownload",
        Some("cab") => "application/vnd.ms-cab-compressed",
        Some("apk") => "application/vnd.android.package-archive",
        _ => "application/octet-stream",
    }
}

#[async_recursion]
pub async fn generate_short_path_url() -> String {
    let mut short_url: Vec<char> = Vec::with_capacity(8);
    let chars: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let numbers: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

    for _ in 0..8 {
        if rand::thread_rng().gen_ratio(1, 2) {
            short_url.push(chars[rand::thread_rng().gen_range(0..52)])
        } else {
            short_url.push(numbers[rand::thread_rng().gen_range(0..10)])
        }
    }

    let short_path_url: String = short_url.iter().collect();
    short_path_url
}
