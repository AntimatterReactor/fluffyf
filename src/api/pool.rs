// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    futures::StreamExt,
    log::error,
    reqwest::{Url, Client, Error},
    serde::Deserialize,
    time::OffsetDateTime,

    crate::connect::methods,

    super::{
        supplement::IdType,
        datetimeformat,
        traits::List,
        post::PostWrapper,
    },
};

pub const POOLS_URL: &'static str = "pools.json";

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Category {
    Series,
    Collection
}

/// An analogue to the pool object returned by sending a `GET`
/// request to *`https://e926.net/pools/<id>.json`*
/// 
/// See [e926's wiki](https://e926.net/wiki_pages/2425#pools) for more detail
#[derive(Debug, Deserialize)]
pub struct Pool {
    pub id: IdType,
    pub name: String,
    #[serde(with = "datetimeformat")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetimeformat")]
    pub updated_at: OffsetDateTime,
    pub creator_id: IdType,
    pub description: String,
    pub is_active: bool,
    pub category: Category,
    pub post_ids: Vec<IdType>,
    pub creator_name: String,
    pub post_count: u16 // No matter how insane an artist is, there is no way there'd be enough
                    // time in the world to create so many post into a pool to overflow this
}

impl Pool {
    /// Creates a new `Pool` by fetching a `url` and
    /// processing the result using `serde`'s `Deserialize`.
    pub async fn new_by_url(client: Client, url: Url) -> Result<Self, Error> {
        Ok(methods::get(client.clone(), url).await?.json::<Self>().await?)
    }

    /// Creates a new `Pool` by fetching an `id` to *`https://e926.net/pools/<id>.json`*.
    /// 
    /// Very similar to [`new_by_url`](Pool::new_by_url).
    /// 
    /// # Example:
    /// 
    /// ```
    /// use fluffyf::api::pools::Pool
    /// 
    /// # let client = create_client(build_header("ostipyroxene", "tCEt2CifHzRzMAcakJuEYpbx"))?;
    /// let pool = Pool::new_by_url(client.clone(), 37853).await?;
    /// ```
    pub async fn new_by_id(client: Client, id: IdType) -> Result<Self, Error> {
        Ok(
            methods::get(client.clone(), format!("https://e926.net/pools/{id}.json")
                .parse().unwrap()).await?.json::<Self>().await?
        )
    }

    pub async fn by_search_first(&self, id: IdType) {

    }

    pub async fn by_search() {

    }

    /// Iteratively fetch all the post in [`post_ids`](Pool::post_ids) (of type
    /// [`IdType`]) and returns [`PostWrapper`].
    /// 
    /// # Example 
    pub async fn get_all_posts(self, client: Client)
        -> Result<Vec<PostWrapper>, Error> {
        
        Ok(futures::stream::iter(
            self.post_ids.into_iter().map(|id| {
                let cc = client.clone();
                async move {
                    match PostWrapper::new_by_id(cc, id).await {
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
