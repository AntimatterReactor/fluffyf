// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    std::{collections::HashMap, error::Error},
    
    bytes::Bytes,
    log::debug,
    reqwest::Client,
    serde::Deserialize,
    tokio::{fs, io::AsyncWriteExt},
    
    crate::RqResult,
};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum Extension {
    #[serde(rename = "jpg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "swf")]
    Swf,
    #[serde(rename = "webm")]
    WebM
}

#[derive(Debug, Deserialize)]
pub struct Alternates {
    pub r#type: String,
    pub width: u32,
    pub height: u32,
    pub urls: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct PostFile {
    /// **Funfact**: The widest image on e621 \[112770\] is 21616 pixels long.
    pub width: u16,

    /// And the tallest \[37061\] is 17700 pixels tall.
    pub height: u16, 

    pub ext: Extension,
    pub size: u64,   // Size is not as clear-cut as dimension because of videos
    pub md5: String,

    /// The URL where the file is hosted on E6.
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PostPreview {
    pub width: u16,
    pub height: u16,

    /// The URL where the preview file is hosted on E6.
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PostSample {
    /// If the post has a sample/thumbnail or not.
    pub has: bool,
    pub height: u16,
    pub width: u16,

    /// The URL where the sample file is hosted on E6.
    pub url: String,

    /// Alternate samples to use, usually only relevant for
    /// videos and specifies different format per sample.
    /// 
    /// The form is Key-Paired Objects where the key is something like `480p`.
    pub alternates: HashMap<String, Alternates>
}

impl PostFile {
    pub async fn get_image_data(&self, client: Client)
        -> RqResult<Bytes> {
        let res = client.get(&self.url).send().await?.bytes().await?;
        debug!("got the bytes");
        Ok(res)
    }
    
    pub async fn write(&self, client: Client, filepath: &str) -> Result<(), Box<dyn Error>> {
        debug!("writing {filepath} using {:#?}", client);
        fs::File::create(filepath).await?
        .write(self.get_image_data(client).await?.as_ref()).await?;
        Ok(())
    }
}
