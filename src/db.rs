use crate::repo::Repo;
use futures::TryStreamExt;
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{bson::doc, options::FindOptions};
use mongodb::{Client, Collection};
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use std::env;
use std::error::Error;

pub struct Db {
    pub client: Client,
}

impl Db {
    pub async fn new() -> Result<Db, Box<dyn Error>> {
        let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

        let options: ClientOptions =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;
        let client = mongodb::Client::with_options(options)?;

        Ok(Db { client })
    }

    pub async fn generate_random_path<T: DeserializeOwned + Unpin + std::marker::Send + Sync>(
        &self,
        collection: &Collection<T>,
    ) -> Result<String, Box<dyn Error>> {
        // Generate random path then check for existing random path
        // before inserting record
        let random_path = crate::utils::generate_random_path();

        let filter = doc! { "random_path": &random_path };
        let find_options = FindOptions::builder().build();
        let mut cursor = collection.find(filter, find_options).await?;

        // if there is an existing timesheet with the random path, generate again
        if let Some(_) = cursor.try_next().await? {
            &self.generate_random_path(collection);
        }

        Ok(random_path)
    }
}
