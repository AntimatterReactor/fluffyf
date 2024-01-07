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
    reqwest::Client,
    serde::Deserialize,
    time::OffsetDateTime,

    crate::RqResult,

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

/// Analogue to `/pools/<id>.json`
/// 
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

impl List for Pool {
    const ENDPOINT: &'static str = "pools";
}

impl Pool {
    pub async fn by_search_first(&self, id: IdType) {

    }

    pub async fn by_search() {

    }

    /// Iteratively fetch all the post in [`post_ids`](Pool::post_ids) (of type
    /// [`IdType`]) and returns [`PostWrapper`].
    /// 
    /// # Example 
    pub async fn get_all_posts(self, client: Client)
        -> RqResult<Vec<PostWrapper>> {
        
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
