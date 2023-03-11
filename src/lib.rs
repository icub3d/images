use futures_util::stream::TryStreamExt;

use std::str;

use lazy_static::lazy_static;
use mongodb::{bson::doc, Client};
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref RE: Regex = Regex::new(r"[^.a-zA-Z0-9-]").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Write(String),
    Database(String),
    Multi(Vec<Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::Write(s) => write!(f, "write: {}", s),
            Error::Database(s) => write!(f, "database: {}", s),
            Error::Multi(ee) => write!(f, "multi: {:?}", ee),
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Media {
    #[serde(rename = "_id")]
    pub filename: String,

    #[serde(rename = "type")]
    pub typ: String,

    pub tags: Vec<String>,
}

pub struct Images {
    client: Client,
    db: String,
}

impl Images {
    pub async fn open(mongo_uri: &str, db: &str) -> Result<Images> {
        let client = match Client::with_uri_str(mongo_uri).await {
            Ok(client) => client,
            Err(err) => return Err(Error::Database(err.to_string())),
        };

        Ok(Self {
            client,
            db: db.to_string(),
        })
    }

    fn sanitize(id: &str) -> String {
        RE.replace_all(id, "-").to_string()
    }

    pub async fn list(&mut self, skip: u64, limit: i64, search: &str) -> Result<Vec<Media>> {
        let mut limit = limit;
        if !(0..=1000).contains(&limit) {
            limit = 10;
        }

        let opts = mongodb::options::FindOptions::builder()
            .sort(doc! {"_id": 1})
            .skip(skip)
            .limit(limit)
            .build();

        let mut filter = doc! {};
        if !search.is_empty() {
            filter = doc! {
                "$text": doc! {
                    "$search": search,
                },
            };
        }

        let mut cursor = match self
            .client
            .database(&self.db)
            .collection::<Media>("media")
            .find(filter, Some(opts))
            .await
        {
            Ok(cursor) => cursor,
            Err(e) => return Err(Error::Database(e.to_string())),
        };
        let mut mm = vec![];
        loop {
            let m = match cursor.try_next().await {
                Ok(r) => r,
                Err(e) => return Err(Error::Database(e.to_string())),
            };
            match m {
                Some(m) => mm.push(m),
                None => break,
            };
        }

        Ok(mm)
    }

    pub async fn remove(&mut self, id: &str) -> Result<()> {
        // sanitize.
        let id = Images::sanitize(id);

        match self
            .client
            .database(&self.db)
            .collection::<Media>("media")
            .delete_one(doc! {"_id": id.clone()}, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(e.to_string())),
        }
    }

    pub async fn get(&mut self, id: &str) -> Result<Option<Media>> {
        // sanitize.
        let id = Images::sanitize(id);
        match self
            .client
            .database(&self.db)
            .collection::<Media>("media")
            .find_one(doc! {"_id": id.clone()}, None)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::Database(e.to_string())),
        }
    }

    pub async fn put(&mut self, m: &mut Media) -> Result<()> {
        // Insert into media.
        let id = Images::sanitize(&m.filename);
        m.filename = id.clone();
        match self
            .client
            .database(&self.db)
            .collection::<Media>("media")
            .insert_one(m, None)
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(Error::Database(e.to_string())),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sanitize() {
        assert_eq!(Images::sanitize("foo/1.jpg"), "foo-1.jpg".to_string());
        assert_eq!(Images::sanitize("foo-1.jpg"), "foo-1.jpg".to_string());
        assert_eq!(Images::sanitize("foo/../1.jpg"), "foo-..-1.jpg".to_string());
    }
}
