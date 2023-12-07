use async_trait::async_trait;
use rusoto_core::credential::{
    AwsCredentials, ContainerProvider, CredentialsError, EnvironmentProvider,
    InstanceMetadataProvider, ProfileProvider, ProvideAwsCredentials,
};
use std::env;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AwsCredentialsProvider {
    prefixed_environment_provider: EnvironmentProvider,
    environment_provider: EnvironmentProvider,
    profile_provider: Option<ProfileProvider>,
    container_provider: Option<ContainerProvider>,
    instance_metadata_provider: Option<InstanceMetadataProvider>,
}

impl AwsCredentialsProvider {
    pub fn new() -> AwsCredentialsProvider {
        let mut all_providers = AwsCredentialsProvider {
            prefixed_environment_provider: EnvironmentProvider::with_prefix("MANTLE_AWS"),
            environment_provider: EnvironmentProvider::default(),
            profile_provider: ProfileProvider::new().ok(),
            container_provider: None,
            instance_metadata_provider: None,
        };

        if let Ok(value) = env::var("MANTLE_AWS_INHERIT_IAM_ROLE") {
            if value == "true" {
                all_providers.container_provider = Some(ContainerProvider::new());
                all_providers
                    .container_provider
                    .expect("ContainerProvider should be initialized")
                    .set_timeout(Duration::from_secs(15));

                all_providers.instance_metadata_provider = Some(InstanceMetadataProvider::new());
                all_providers
                    .instance_metadata_provider
                    .expect("InstanceMetadataProvider should be initialized")
                    .set_timeout(Duration::from_secs(15));
            }
        }

        return all_providers;
    }
}

async fn chain_provider_credentials(
    provider: AwsCredentialsProvider,
) -> Result<AwsCredentials, CredentialsError> {
    if let Ok(creds) = provider.prefixed_environment_provider.credentials().await {
        return Ok(creds);
    }
    if let Ok(creds) = provider.environment_provider.credentials().await {
        return Ok(creds);
    }
    if let Some(ref profile_provider) = provider.profile_provider {
        if let Ok(creds) = profile_provider.credentials().await {
            return Ok(creds);
        }
    }
    if let Ok(creds) = provider.container_provider.credentials().await {
        return Ok(creds);
    }
    if let Ok(creds) = provider.instance_metadata_provider.credentials().await {
        return Ok(creds);
    }
    Err(CredentialsError::new(
        "Couldn't find AWS credentials in environment, credentials file, or instance/container IAM role.",
    ))
}

#[async_trait]
impl ProvideAwsCredentials for AwsCredentialsProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        chain_provider_credentials(self.clone()).await
    }
}
