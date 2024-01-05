// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

mod common;

use {
    fluffyf::connect::{methods, blocking},
    reqwest::blocking::multipart::Form,

    crate::common::build_header,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = blocking::create_client(build_header("ostipyroxene", "tCEt2CifHzRzMAcakJuEYpbx"))?;
    println!("{:#?}", client);
    let f = Form::new().file("upload[file]", "/home/jaded/codes/fluffyf/7596994.png")?; // TODO: complete with data
    println!("{:#?}", f);
    let r = methods::blocking::post(client, "https://e621.net/posts.json".parse()?, f)?;
    println!("{:#?}", r.text());
    Ok(())
}
