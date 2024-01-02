pub mod score;
pub mod flags;
pub mod rating;
pub mod relations;

use async_trait::async_trait;
use reqwest::{Url, Client, Error};
use serde::{Deserialize, de::Visitor};
use time::OffsetDateTime;

use crate::connect::method;

use self::{relations::Relations, score::Score, flags::Flags, rating::Rating};

use super::{tags::{TagObject, TagType}, file::{FileObject, PreviewObject, SampleObject}, supplement::IdType, traits::*, datetimeformat};

pub const POSTS_URL: &'static str = "posts.json";

#[derive(Debug, Deserialize)]
pub struct PostObject {
    pub id: IdType,
    #[serde(with = "datetimeformat")]
    created_at: OffsetDateTime,
    #[serde(with = "datetimeformat")]
    updated_at: OffsetDateTime,
    pub file: FileObject,
    preview: PreviewObject,
    sample: SampleObject,
    score: Score,
    tags: TagObject,
    locked_tags: TagType,
    change_seq: u32,
    flags: Flags,
    rating: Rating,
    fav_count: u32,
    sources: Vec<String>,
    pools: Vec<IdType>,
    relationships: Relations,
    approver_id: Option<IdType>,
    uploader_id: IdType,
    description: String,
    comment_count: u16,
    is_favorited: bool,
    has_notes: bool,
    duration: Option<f32>
}

impl PostObject {


    pub async fn vote(&self, client: Client) {
        //method::post(client, , )
    }

    pub async fn vote_by_id(id: IdType) {

    }

    pub async fn vote_by_url(url: Url) {

    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum PostObjectWrapper {
    Post(PostObject)
}

impl PostObjectWrapper {
    pub async fn new_by_url(client: &Client, url: Url) -> Result<Self, Error> {
        Ok(method::get(client, url).await?.json::<Self>().await?)
    }

    pub async fn new_by_id(client: &Client, id: IdType) -> Result<Self, Error> {
        Ok(
            method::get(client, format!("https://e621.net/posts/{id}.json")
                .parse().unwrap()).await?.json::<Self>().await?
        )
    }
}



#[derive(Debug, Deserialize)]
pub struct Posts {
    posts: Vec<PostObject>
}
