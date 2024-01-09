// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{ArgAction, ArgMatches, Arg};
use log::info;

mod common;

use {
    std::{
        error::Error as stdError,
        path::PathBuf,
        fs,
    },
    clap::{Command, arg},
    common::build_header,
    futures::{StreamExt, stream, executor::block_on},
    fluffyf::{connect::*, api::{PostWrapper, Pool, List, IdType}},
};

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_NAME"))
        .about("A fast asynchronous E621/E926 post(s)/pool(s) downloader")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_names(["NAME", "APIKEY"])
                .help("Authenticate using NAME and APIKEY")
                .required(true)
            )
        .subcommand(
            Command::new("posts")
                .alias("post")
                .about("Fetch and downloads posts")
                .allow_missing_positional(true)
                .arg_required_else_help(true)
                .arg(
                    arg!(-o --output <PATH> "Sets the output path, defaults to current directory")
                        .default_value(".")
                        .value_parser(clap::value_parser!(PathBuf))
                )
                .arg(
                    arg!(<ID> ... "The ID of posts to download")
                        .required(true)
                        .value_parser(clap::value_parser!(IdType))
                )
        )
        .subcommand(
            Command::new("pools")
                .alias("pool")
                .about("Fetch and downloads pools")
                .allow_missing_positional(true)
                .arg_required_else_help(true)
                .arg(
                    arg!(-o --output <PATH> "Sets the output path, defaults to current directory")
                        .default_value(".")
                        .value_parser(clap::value_parser!(PathBuf))
                )
                .arg(
                    arg!(-N --name "If used, make the pool folders name from pool names instead of from ID")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    arg!(<ID> ... "The ID of pools to download")
                        .required(true)
                        .value_parser(clap::value_parser!(IdType))
                )
        )
}

#[tokio::main]
async fn download_posts(client: reqwest::Client, path: &PathBuf, posts: Vec<&IdType>) -> Result<(), Box<dyn stdError>> {
    futures::stream::iter(posts.iter().map(|id| {
        let cc = client.clone();
        let p = path.clone();
        async move {
            match PostWrapper::new_by_id(cc.clone(), **id).await {
                Ok(PostWrapper::Post(post)) => {
                    match post.file.get_image_data(cc).await {
                        Ok(b) => {
                            fs::write(format!("{}/{}.png", p.display(), id), b.as_ref())
                            .unwrap();
                        },
                        Err(_) => panic!()
                    }
                }
                Err(_) => println!("error: post of id {} not found", id)
            }
        }
    })).buffer_unordered(20).collect::<Vec<()>>().await;
    Ok(())
}

fn main() -> Result<(), Box<dyn stdError>> {
    // Starts the logger
    // TODO: change the logger implementation to env_logger
    simple_logger::SimpleLogger::new().init().unwrap();

    let matches = cli().get_matches();

    let auth = 
        matches.get_many::<String>("user").into_iter().flatten().collect::<Vec<_>>();
        
    let client = create_client(build_header(auth[0], auth[1]))
        .expect("2 arguments guaranteed in clap");

    match matches.subcommand() {
        Some(("posts", sm)) => {
            let path = sm
                .get_one::<PathBuf>("output")
                .expect("defaulted in clap");

            let ids = sm
                .get_many::<IdType>("ID")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            info!("These ids: {:?} to here: {:?}", ids, path);
            let _ = download_posts(client, path, ids);
        }
        Some(("pools", sm)) => {
            let path = sm
                .get_one::<PathBuf>("output")
                .expect("defaulted in clap");
            
            let name = sm.get_flag("name");

            let ids = sm
                .get_many::<IdType>("ID")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            println!("These ids: {:?} to here: {:?} and name is {}", ids, path, name);
        }
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }    

    Ok(())
}
