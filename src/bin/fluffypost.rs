mod common;

use fluffyf::connect::{method, blocking};
use reqwest::blocking::multipart::Form;

use crate::common::build_header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = blocking::create_client(build_header("ostipyroxene", "tCEt2CifHzRzMAcakJuEYpbx"))?;
    println!("{:#?}", client);
    let f = Form::new().file("upload[file]", "/home/jaded/codes/fluffyf/7596994.png")?; // TODO: complete with data
    println!("{:#?}", f);
    let r = method::blocking::post(client, "https://e621.net/posts.json".parse()?, f)?;
    println!("{:#?}", r.text());
    Ok(())
}
