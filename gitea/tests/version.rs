use anyhow::Result;
use gitea::Client;

#[tokio::test]
async fn version() -> Result<()> {
    let cli = Client::new(
        std::env::var("GITEA_SERVER")?,
        std::env::var("DOMO_GITEA_TOKEN")?,
        "gitea/tests",
    )?;

    let vers = cli.version().await?;
    println!("gitea version {}", vers.version);

    Ok(())
}
