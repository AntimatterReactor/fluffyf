// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use fluffyf::connect::{methods, create_client, build_header, encode_login};
use reqwest::Url;
use std::error::Error as stdError;

mod posts {
    use fluffyf::api::pools::PoolObject;

    use super::*;
    
    #[tokio::test]
    async fn test_get() -> Result<(), Box<dyn stdError>> {
        let client = create_client(build_header(encode_login("ostipyroxene", "tCEt2CifHzRzMAcakJuEYpbx")))?;
        let url = Url::parse("https://e621.net/pools/37853.json")?;
        let res = methods::get(client, url) .await?.json::<PoolObject>().await?;
        println!("{res:#?}");
        Ok(())
    }
}
