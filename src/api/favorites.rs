use serde::Deserialize;

use super::posts::PostObject;

#[derive(Debug, Deserialize)]
pub struct FavObject {
    pub posts: Vec<PostObject>
}

impl FavObject {
    
}
