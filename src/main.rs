extern crate dotenv;

use std::env;
use std::error::Error;
use bson::doc;

use dotenv::dotenv;
use mongodb::{Client, options::ClientOptions};
use mongodb::options::ResolverConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("DB_URL").expect("You must set the DB_URL environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    client.database("test").collection("test").insert_one(doc! { "name": "test" }, None).await?;

    Ok(())
}
