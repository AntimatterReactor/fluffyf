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

/// An analogue to the pool object returned by sending a `GET`
/// request to *`https://e621.net/pools/<id>.json`*
/// 
/// See [e621's wiki](https://e621.net/wiki_pages/2425#pools>) for more detail
#[derive(Debug, Deserialize)]
pub struct PoolObject {
    /// The id number of the pool
    pub id: IdType,

    /// The name of the pool
    pub name: String,

    /// The time the pool was created in the format of `YYYY-MM-DDTHH:MM:SS.MS+00:00`
    #[serde(with = "datetimeformat")]
    pub created_at: OffsetDateTime,

    /// The time the pool was updated in the format of `YYYY-MM-DDTHH:MM:SS.MS+00:00`
    #[serde(with = "datetimeformat")]
    pub updated_at: OffsetDateTime,

    /// The id of the user that created the pool
    pub creator_id: IdType,

    /// The description of the pool
    pub description: String,

    /// If the pool is active and still getting posts added 
    pub is_active: bool,

    /// Can be `Series` or `Collection`
    pub category: Category,

    /// A `Vec` of posts ids in the pool
    pub post_ids: Vec<IdType>,

    /// The name of the user that created the pool
    pub creator_name: String,
    
    /// The amount of posts in the pool
    pub post_count: u16 // No matter how insane an artist is, there is no way there'd be enough
                    // time in the world to create so many post into a pool to overflow this
}

impl PoolObject {
    /// Creates a new `PoolObject` by fetching a `url` and
    /// processing the result using `serde`'s `Deserialize`
    pub async fn new_by_url(client: Client, url: Url) -> Result<Self, Error> {
        Ok(method::get(client.clone(), url).await?.json::<Self>().await?)
    }

    /// Creates a new `PoolObject` by fetching an `id` to *`https://e621.net/pools/<id>.json`*
    /// 
    /// Very similar to [`new_by_url`](PoolObject::new_by_url)
    /// 
    /// # Example:
    /// 
    /// ```
    /// use fluffyf::api::pools::PoolObject
    /// 
    /// # let client = create_client(build_header("ostipyroxene", "tCEt2CifHzRzMAcakJuEYpbx"))?;
    /// let pool = PoolObject::new_by_url(client.clone(), 37853).await?;
    /// ```
    pub async fn new_by_id(client: Client, id: IdType) -> Result<Self, Error> {
        Ok(
            method::get(client.clone(), format!("https://e621.net/pools/{id}.json")
                .parse().unwrap()).await?.json::<Self>().await?
        )
    }

    pub async fn by_search_first(&self, id: IdType) {

    }

    pub async fn by_search() {

    }

    /// Iteratively fetch all the post in [`post_ids`](PoolObject::post_ids) (of type
    /// [`IdType`]) and returns [`PostObjectWrapper`]
    /// 
    /// # Example 
    pub async fn get_all_posts(self, client: Client)
        -> Result<Vec<PostObjectWrapper>, Error> {
        
        Ok(futures::stream::iter(
            self.post_ids.into_iter().map(|id| {
                let cc = client.clone();
                async move {
                    match PostObjectWrapper::new_by_id(cc, id).await {
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
