// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    std::cmp::Ordering,

    reqwest::{Url, Client, Error},
    serde::Deserialize,
    time::OffsetDateTime,

    crate::connect::methods,
    
    super::{
        tags::{Tags, TagType},
        file::{PostFile, Preview, Sample},
        supplement::IdType,
        traits::*,
        datetimeformat,
    },
};

pub const POSTS_URL: &'static str = "posts.json";

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum Rating {
    #[serde(rename = "s")]
    Safe,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "e")]
    Explicit
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Flags {
    /// If the post is pending approval.
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub deleted: bool
}

#[derive(Debug, Deserialize)]
pub struct Relations {
    pub parent_id: Option<IdType>,
    pub has_children: bool,
    pub has_active_children: bool,
    pub children: Vec<IdType>
}

#[derive(Debug, Eq, Deserialize)]
pub struct Score {
    pub up: i32,
    pub down: i32,
    pub total: i32,

    #[serde(default)]
    pub our_score: i32,
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}

impl Score {
    fn count_total(&mut self) -> i32 {
        self.total = self.up + self.down;
        self.total
    }
}

#[derive(Debug, Deserialize)]
pub struct Post {
    pub id: IdType,
    #[serde(with = "datetimeformat")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetimeformat::option")]
    pub updated_at: Option<OffsetDateTime>,
    pub file: PostFile,
    pub preview: Preview,
    pub sample: Sample,
    pub score: Score,
    pub tags: Tags,
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

impl Post {


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
pub enum PostWrapper {
    Post(Post)
}

impl List for PostWrapper {
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

#[derive(Debug, Deserialize)]
pub struct Posts {
    pub posts: Vec<Post>
}
