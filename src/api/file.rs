use std::{collections::HashMap, error::Error, sync::{Mutex, Arc}};

use bytes::Bytes;
use log::debug;
use reqwest::{Client, Url};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Deserialize)]
pub struct AltObject {
    r#type: String,
    width: u32,
    height: u32,
    urls: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct FileObject {
    width: u16,  // The widest image on e621 [112770] is 21616 pixels long and
    height: u16, // the tallest [37061] is 17700 pixels tall, much less than 2^16-1
    pub ext: String,
    size: u64,   // Size is not as clear-cut as dimension because of videos
    md5: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PreviewObject {
    width: u16,
    height: u16,
    url: Option<String>,
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
