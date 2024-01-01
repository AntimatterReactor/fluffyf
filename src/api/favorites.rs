use serde::Deserialize;

use super::post::PostObject;

#[derive(Debug, Deserialize)]
pub struct FavObject {
    posts: Vec<PostObject>
}

impl FavObject {
    
}
