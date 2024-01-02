mod common;

use std::{error::Error as stdError, time::Duration, sync::{Arc, Mutex}};
use common::build_header;
use futures::StreamExt;
use reqwest::Url;
use fluffyf::{connect::*, api::{posts::{PostObject, PostObjectWrapper}, favorites::FavObject, pools::PoolObject}};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn stdError>> {
    simple_logger::SimpleLogger::new().init().unwrap();
    let client = create_client(build_header("ostipyroxene", "tCEt2CifHzRzMAcakJuEYpbx"))?;
    let pool = PoolObject::new_by_id(&client, 37853).await?;
    println!("{:#?}", pool);
    let posts = pool.get_all_posts(&client).await?;
    futures::stream::iter(posts.iter().map(|postw| {
        let cc = client.clone();
        async move {
            match postw {
                PostObjectWrapper::Post(post) => {
                    match post.file.get_image_data(cc).await {
                        Ok(b) => {
                            File::create(format!("volleyball/{}.png", post.id))
                            .await.unwrap()
                            .write(b.as_ref())
                            .await.unwrap();
                        },
                        Err(_) => panic!()
                    }
                }
            }
        }
    })).buffer_unordered(20).collect::<Vec<()>>().await;
    Ok(())
}
