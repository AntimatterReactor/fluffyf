// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

pub(crate) trait Search {
    // async fn query
}

pub enum PageSearchType {
    Page(u32),
    After(u32),
    Before(u32)
}

pub struct SearchPosts {

}

pub struct SearchPools {

}

pub struct SearchTags {

}

pub struct SearchTagAliases {

}

pub struct SearchNotes {

}

pub struct SearchPostFlags {

}

