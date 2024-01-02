use serde::Deserialize;

use super::posts::PostObject;

#[derive(Debug, Deserialize)]
pub struct FavObject {
    posts: Vec<PostObject>
}

impl FavObject {
    
}
