use mongodb::{Client, Collection};
use bson::Document;
use std::error::Error;

use chrono::{TimeZone, Utc};
use mongodb::bson::doc;


pub async fn get_movies_collection(client: &Client) -> Result<Collection<Document>, Box<dyn Error>> {
    let db = client.database("sample_mflix");
    Ok(db.collection("movies"))
}

pub async fn insert_movie(client: &Client) -> Result<(), Box<dyn Error>> {
  let movies = get_movies_collection(client).await?;

  let new_doc = doc! {
     "title": "Parasite",
     "year": 2020,
     "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
     "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
  };

  let insert_result = movies.insert_one(new_doc.clone(), None).await?;
  println!("New document ID: {}", insert_result.inserted_id);

  Ok(())
}