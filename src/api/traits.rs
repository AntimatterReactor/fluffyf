// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use reqwest::{Url, Client};

use super::supplement::IdType;

#[async_trait]
pub trait List {


}

#[async_trait]
pub trait Create {

}

#[async_trait]
pub trait Update {

}

#[async_trait]
pub trait Delete {

}

#[async_trait]
pub trait Revert {
    
}

#[async_trait]
pub trait Vote {
   async fn vote(&self, client: Client) {
        
    }

    async fn vote_by_id(id: IdType) {

    }

    async fn vote_by_url(url: Url) {

    }
}
