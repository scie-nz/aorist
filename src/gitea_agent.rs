use lib::utils::get_data_setup;
use lib::error::AoristError;
use lib::ranger::TRangerEntity;
use lib::user::TGiteaEntity;

#[tokio::main]
async fn main() -> Result<(), AoristError> {
    const APP_USER_AGENT: &'static str =
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let gitea_client = gitea::Client::new(
        "http://localhost:30807".into(),
        "2b44b07e042ee9fe374e3eeebd2c9098468b5774".into(),
        APP_USER_AGENT,
    )
    .unwrap();

    let ranger_client = ranger::RangerClient::new(
        "http://localhost:30800".into(),
        "admin".to_string(),
        "G0powerRangers".to_string(),
    )
    .unwrap();

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
    for user in setup.get_users_mut().unwrap() {
        TGiteaEntity::enforce(user, &gitea_client).await.unwrap();
        TRangerEntity::enforce(user, &ranger_client).await.unwrap();
    }
    Ok(())
}
