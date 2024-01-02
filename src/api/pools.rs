use futures::StreamExt;
use log::error;
use reqwest::{Url, Client, Error};
use serde::{Deserialize, de::Visitor};
use time::OffsetDateTime;

use crate::connect::method;

use super::{supplement::IdType, datetimeformat, traits::List, posts::{PostObject, PostObjectWrapper}};

pub const POOLS_URL: &'static str = "pools.json";

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Series,
    Collection
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        struct CategoryVisitor;
        
        impl<'de> Visitor<'de> for CategoryVisitor {
            type Value = Category;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string equal to either `series` or `collection`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                match v {
                    "series" => Ok(Self::Value::Series),
                    "collection" => Ok(Self::Value::Collection),
                    _ => Err(E::custom(format!("unexpected category {v}")))
                }
            }
        }
        deserializer.deserialize_str(CategoryVisitor)
    }
}

// TODO: impl Serialize for Category

#[derive(Debug, Deserialize)]
pub struct PoolObject {
    id: IdType,
    name: String,
    #[serde(with = "datetimeformat")]
    created_at: OffsetDateTime,
    #[serde(with = "datetimeformat")]
    updated_at: OffsetDateTime,
    creator_id: IdType,
    description: String,
    is_active: bool,
    category: Category,
    post_ids: Vec<IdType>,
    creator_name: String,
    post_count: u16 // No matter how insane an artist is, there is no way there'd be enough
                    // time in the world to create so many post into a pool to overflow this
}

impl PoolObject {
    pub async fn new_by_url(client: &Client, url: Url) -> Result<Self, Error> {
        Ok(method::get(client, url).await?.json::<Self>().await?)
    }

    pub async fn new_by_id(client: &Client, id: IdType) -> Result<Self, Error> {
        Ok(
            method::get(client, format!("https://e621.net/pools/{id}.json")
                .parse().unwrap()).await?.json::<Self>().await?
        )
    }

    pub async fn by_search_first(&self, id: IdType) {

    }

    pub async fn by_search() {

    }

    pub async fn get_all_posts(self, client: &Client)
        -> Result<Vec<PostObjectWrapper>, Error> {
        
        Ok(futures::stream::iter(
            self.post_ids.into_iter().map(|id| {
                async move {
                    match PostObjectWrapper::new_by_id(client, id).await {
                        Ok(x) => x,
                        Err(e) => {
                            error!("Error while getting all posts: {:#?}", e);
                            panic!();
                        }
                    }
                }
            }
        )).buffer_unordered(20).collect::<Vec<_>>().await)
    }
}
