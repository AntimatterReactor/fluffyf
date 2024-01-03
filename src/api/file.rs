// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{collections::HashMap, error::Error, sync::{Mutex, Arc}};

use bytes::Bytes;
use log::debug;
use reqwest::{Client, Url};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Deserialize)]
pub struct AltObject {
    pub r#type: String,
    pub width: u32,
    pub height: u32,
    pub urls: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct FileObject {
    pub width: u16,  // The widest image on e621 [112770] is 21616 pixels long and
    pub height: u16, // the tallest [37061] is 17700 pixels tall, much less than 2^16-1
    pub ext: String,
    pub size: u64,   // Size is not as clear-cut as dimension because of videos
    pub md5: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PreviewObject {
    pub width: u16,
    pub height: u16,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SampleObject {
    pub has: bool,
    pub height: u16,
    pub width: u16,
    pub url: Option<String>,
    pub alternates: HashMap<String, AltObject>
}

impl FileObject {
    pub async fn get_image_data(&self, client: Client)
        -> Result<Bytes, reqwest::Error> {
        let res = client.get(&self.url).send().await?.bytes().await?;
        debug!("got the bytes");
        Ok(res)
    }
    
    pub async fn write(&self, client: Client, filepath: &str) -> Result<(), Box<dyn Error>> {
        debug!("writing {filepath} using {:#?}", client);
        File::create(filepath).await?
        .write(self.get_image_data(client).await?.as_ref()).await?;
        Ok(())
    }
}
