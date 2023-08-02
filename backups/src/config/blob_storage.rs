use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

#[derive(Clone, Debug)]
pub struct AwsS3Storage {}

impl AwsS3Storage {
    pub fn load_from_env() -> Result<Self, anyhow::Error> {
        std::env::var("AWS_ACCESS_KEY_ID")?;
        std::env::var("AWS_SECRET_ACCESS_KEY")?;

        Ok(Self {})
    }

    pub async fn client(&self) -> Client {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        Client::new(&config)
    }
}
