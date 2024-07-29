mod create;

use create::{get_movies_collection, insert_movie};
use std::env;
use tokio;
use futures_util::stream::StreamExt;
use mongodb::{options::{ClientOptions, ResolverConfig}, Client};
// use trust_dns_resolver::config::ResolverConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;

    match get_movies_collection(&client).await {
        Ok(movies) => {
            println!("Successfully got the movies collection!");

            // 查找所有文档
            // let mut cursor = movies.find(None, None).await.unwrap();
            // while let Some(result) = cursor.next().await {
            //     match result {
            //         Ok(document) => {
            //             println!("{:?}", document);
            //         }
            //         Err(e) => {
            //             println!("Error retrieving document: {}", e);
            //         }
            //     }
            // }
        },
        Err(e) => {
            println!("Failed to get the movies collection: {}", e);
        }
    }

    // 插入数据到 movie 集合中
    insert_movie(&client).await?;

    Ok(())
}