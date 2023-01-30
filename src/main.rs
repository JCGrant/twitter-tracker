use anyhow::Context;
use chrono::Local;
use dotenv::dotenv;
use serde::Deserialize;
use twitter_v2::authorization::BearerToken;
use twitter_v2::query::UserField;
use twitter_v2::TwitterApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    let config = envy::from_env::<Config>()?;
    let auth = BearerToken::new(config.twitter_bearer_token);
    let user = TwitterApi::new(auth)
        .get_user(config.twitter_user_id)
        .user_fields([UserField::PublicMetrics])
        .send()
        .await?
        .into_data()
        .expect("this user should exist");
    let now = Local::now();
    println!(
        "{},{}",
        now.format("%Y-%m-%d %H:%M:%S"),
        user.public_metrics
            .context("no public metrics")?
            .followers_count
    );
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Config {
    twitter_bearer_token: String,
    twitter_user_id: u64,
}

