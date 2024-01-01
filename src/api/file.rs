use std::{str::Bytes, collections::HashMap};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AltObject {
    r#type: String,
    width: u32,
    height: u32,
    urls: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct FileObject {
    width: u32,  // According to the png standard, the maximum width and
    height: u32, // height is one that can be contained in 4 byte uint (u32)
    ext: String,
    size: u32,
    md5: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct PreviewObject {
    width: u32,
    height: u32,
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SampleObject {
    has: bool,
    height: u32,
    width: u32,
    url: Option<String>,
    alternates: HashMap<String, AltObject>
}

impl FileObject {
    // async fn get_image_data(&self) -> Bytes {
        
    // }
}
