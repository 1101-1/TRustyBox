use std::env;

use async_recursion::async_recursion;

use crate::tools::generate_short_path_url;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection,
};

pub async fn insert_to_mongodb(
    path_download: &String,
    new_filename: &String,
    first_name: &str,
    mut short_path_url: String,
) -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse(env::var("MONGO").expect("Unexpected error")).await?;

    let client = Client::with_options(client_options)?;

    let db = client.database(
        env::var("DATABASE_NAME")
            .expect("Unexpected error")
            .as_str(),
    );

    let collection = db.collection(
        env::var("COLLECTION_NAME")
            .expect("Unexpected error")
            .as_str(),
    );

    if let Some(_url) = collection
        .find_one(doc! {"short_url": &short_path_url}, None)
        .await
        .unwrap()
    {
        short_path_url = find_dublicate(generate_short_path_url().await).await;
    };

    let document = doc! {
        "path": path_download,
        "first_name": first_name,
        "new_filename": new_filename,
        "short_url": short_path_url,
    };
    collection.insert_one(document, None).await?;

    Ok(())
}

#[async_recursion]
async fn find_dublicate(short_url: String) -> String {
    let client_options = ClientOptions::parse(env::var("MONGO").expect("Unexpected error"))
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();

    let db = client.database(
        env::var("DATABASE_NAME")
            .expect("Unexpected error")
            .as_str(),
    );

    let collection: Collection<Document> = db.collection(
        env::var("COLLECTION_NAME")
            .expect("Unexpected error")
            .as_str(),
    );

    if let Some(_url) = collection
        .find_one(doc! {"short_url": &short_url}, None)
        .await
        .unwrap()
    {
        find_dublicate(generate_short_path_url().await).await;
    }

    short_url
}

pub async fn get_name_and_path_of_file(
    bd_short_url: String,
) -> mongodb::error::Result<(String, String, String)> {
    let client_options = ClientOptions::parse(env::var("MONGO").expect("Unexpected error")).await?;

    let client = Client::with_options(client_options)?;

    let db = client.database(
        env::var("DATABASE_NAME")
            .expect("Unexpected error")
            .as_str(),
    );
    let collection: Collection<Document> = db.collection(
        env::var("COLLECTION_NAME")
            .expect("Unexpected error")
            .as_str(),
    );

    if let Some(doc) = collection
        .find_one(doc! {"short_url": bd_short_url}, None)
        .await?
    {
        let path = doc.get_str("path").unwrap().to_string();
        let first_name = doc.get_str("first_name").unwrap().to_string();
        let changed_name = doc.get_str("new_filename").unwrap().to_string();
        return Ok((path, first_name, changed_name));
    } else {
        return Err(mongodb::error::Error::from(tokio::io::Error::new(
            tokio::io::ErrorKind::Other,
            "FILE not found or URL doesn't exist",
        )));
    }
}