// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::Deserialize;

use super::posts::PostObject;

#[derive(Debug, Deserialize)]
pub struct FavObject {
    pub posts: Vec<PostObject>
}

impl FavObject {
    
}
