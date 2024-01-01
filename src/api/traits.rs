use async_trait::async_trait;

#[async_trait]
pub trait List {
    async fn get(&self) {

    }
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
