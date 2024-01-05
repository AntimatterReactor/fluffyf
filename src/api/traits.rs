// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    reqwest::{Url, Client, Error},
    serde::de::DeserializeOwned,

    crate::connect::methods,

    super::supplement::IdType,
};

pub trait List where Self: Sized + DeserializeOwned {
    async fn new_by_url(client: Client, url: Url) -> Result<Self, Error> {
        Ok(methods::get(client, url).await?.json::<Self>().await?)
    }

    async fn new_by_id(client: Client, id: IdType) -> Result<Self, Error> {
        Ok(
            methods::get(client, format!("https://e621.net/posts/{id}.json")
                .parse().unwrap()).await?.json::<Self>().await?
        )
    }
}

pub trait Create {

}

pub trait Update {

}

pub trait Delete {

}

pub trait Revert {
    
}

pub trait Vote {
   async fn vote(&self, client: Client) {
        
    }

    async fn vote_by_id(id: IdType) {

    }

    async fn vote_by_url(url: Url) {

    }
}
