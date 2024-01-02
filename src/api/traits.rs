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
