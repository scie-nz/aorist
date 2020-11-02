use lib::datasets::get_data_setup;
use tokio::prelude::*;
use gitea::Error;

use reqwest::header;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
use thiserror::Error;
use lib::user::TGiteaEntity;


#[tokio::main]
async fn main() -> Result<(), Error> {

    const APP_USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let client = gitea::Client::new(
        "http://localhost:30807".into(),
        "1e8d6b15b3696f45f213fd4153a6516fc06852f8".into(),
        APP_USER_AGENT
    ).unwrap();

    /*
    let whoami = client.whoami().await?;
    println!("{}", whoami.login);
    let res = client.cli.get(&format!("{}/api/v1/admin/users?&page=1&limit=100", client.base_url)).send().await?.error_for_status()?;
    println!("{}", res.text().await?);
    let gitea_users = client.list_all_users().await?;
    for user in gitea_users {
        println!("{}", user.login);
    }

    println!("user {} = {}", "bogdan", client.check_exists_username("bogdan".to_string()).await?);
    println!("user {} = {}", "gitadmin", client.check_exists_username("gitadmin".to_string()).await?);
    */
    let mut setup = get_data_setup();
    for user in setup.get_mutable_users() {
        user.enforce(&client).await.unwrap();
    }
    Ok(())
}
