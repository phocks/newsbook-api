use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use mongodb::bson::{self, doc, Bson};
use std::env;
use std::error::Error;
use tokio;
use warp::Filter;

use couch_rs::document::DocumentCollection;
use couch_rs::types::find::FindQuery;
use serde_json::Value;

const DB_HOST: &str = "http://localhost:5984";
const TEST_DB: &str = "test_db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = couch_rs::Client::new(DB_HOST, "admin", "password")?;
    let db = client.db(TEST_DB).await?;
    let find_all = FindQuery::find_all();
    let docs = db.find_raw(&find_all).await?;
    
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Load the MongoDB connection string from an environment variable:
//     let client_uri = "mongodb://localhost:27017";
//     //   env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
//     // A Client is needed to connect to MongoDB:
//     let client = mongodb::Client::with_uri_str(client_uri.as_ref()).await?;
//     // Print the databases in our MongoDB cluster:
//     println!("Databases:");

//     let databases = client.list_database_names(None, None).await?;

//     for name in databases {
//         println!("- {}", name);
//     }

//     let new_doc = doc! {
//        "title": "Parasite",
//        "year": 2020,
//        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
//        "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
//     };

//     println!("{}", new_doc);

//     Ok(())
// }
