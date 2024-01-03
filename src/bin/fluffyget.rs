// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

mod common;

use std::error::Error as stdError;
use clap::{command, Parser, arg, Subcommand};
use common::build_header;
use futures::StreamExt;
use fluffyf::{connect::*, api::{posts::PostObjectWrapper, pools::PoolObject}};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn download_pool() -> Result<(), Box<dyn stdError>> {
    let client = create_client(build_header("ostipyroxene", "tCEt2CifHzRzMAcakJuEYpbx"))?;
    let pool = PoolObject::new_by_id(client.clone(), 37853).await?;
    println!("{:#?}", pool);
    let posts = pool.get_all_posts(client.clone()).await?;
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

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     /// Optional name to operate on
//     name: Option<String>,

//     /// Sets a custom config file
//     #[arg(short, long, value_name = "FILE")]
//     config: Option<PathBuf>,

//     /// Turn debugging information on
//     #[arg(short, long, action = clap::ArgAction::Count)]
//     debug: u8,

//     #[command(subcommand)]
//     command: Option<Commands>,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// does testing things
//     Test {
//         /// lists test values
//         #[arg(short, long)]
//         list: bool,
//     },
// }

fn main() -> Result<(), Box<dyn stdError>> {
    // Starts the logger
    // TODO: change the logger implementation to env_logger
    simple_logger::SimpleLogger::new().init().unwrap();

    Ok(())
}
