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
    pub created_at: OffsetDateTime,
    #[serde(with = "datetimeformat")]
    pub updated_at: OffsetDateTime,
    pub file: FileObject,
    pub preview: PreviewObject,
    pub sample: SampleObject,
    pub score: Score,
    pub tags: TagObject,
    pub locked_tags: TagType,
    pub change_seq: u32,
    pub flags: Flags,
    pub rating: Rating,
    pub fav_count: u32,
    pub sources: Vec<String>,
    pub pools: Vec<IdType>,
    pub relationships: Relations,
    pub approver_id: Option<IdType>,
    pub uploader_id: IdType,
    pub description: String,
    pub comment_count: u16,
    pub is_favorited: bool,
    pub has_notes: bool,
    pub duration: Option<f32>
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
    pub async fn new_by_url(client: Client, url: Url) -> Result<Self, Error> {
        Ok(method::get(client, url).await?.json::<Self>().await?)
    }

    pub async fn new_by_id(client: Client, id: IdType) -> Result<Self, Error> {
        Ok(
            method::get(client, format!("https://e621.net/posts/{id}.json")
                .parse().unwrap()).await?.json::<Self>().await?
        )
    }
}



#[derive(Debug, Deserialize)]
pub struct Posts {
    pub posts: Vec<PostObject>
}
