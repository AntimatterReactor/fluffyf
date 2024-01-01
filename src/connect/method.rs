use log::debug;
use reqwest::{Url, Client, Error as rqError, Response};

pub async fn get(client: Client, url: Url) -> Result<Response, rqError> {
    debug!("Fetching {url} using get...");
    let res = client.get(url).send().await?.error_for_status()?;
    debug!("Got {res:#?}");
    Ok(res)
}
