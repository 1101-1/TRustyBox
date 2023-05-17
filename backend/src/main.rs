use axum_extra::routing::RouterExt;
use dotenv::dotenv;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post, Router},
};
use std::env;

use crate::{file_actions::upload_file, tools::http_tools::get_favicon::favicon};
use crate::upload_file::upload_file;
use crate::{
    file_actions::download_file::{download_file, download_file_with_aes},
    tools::http_tools::cors::create_cors,
};

mod crypt;
mod db;
mod file_actions;
mod tools;

const MAX_FILE_SIZE: usize = 15 * 1024 * 1024;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", post(upload_file))
        .layer(DefaultBodyLimit::max(MAX_FILE_SIZE))
        .layer(create_cors())
        .route_with_tsr("/favicon.ico", get(favicon))
        .route_with_tsr("/:path", get(download_file))
        .route_with_tsr("/:path/:aes_key", get(download_file_with_aes));

    let addr = env::var("SERVER_ADDR")
        .expect("ADDR NOT FOUND")
        .parse()
        .expect("Address is incorrect");

    tracing::log::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start the server");
}
