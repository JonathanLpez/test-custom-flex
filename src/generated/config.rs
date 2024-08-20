use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(alias = "oauthApiKey")]
    pub oauth_api_key: String,
    #[serde(alias = "oauthClientSecret")]
    pub oauth_client_secret: String,
    #[serde(alias = "oauthGrantType")]
    pub oauth_grant_type: String,
    #[serde(alias = "oauthPasswordBody")]
    pub oauth_password_body: String,
    #[serde(alias = "oauthPasswordHeader")]
    pub oauth_password_header: String,
    #[serde(alias = "oauthPath")]
    pub oauth_path: String,
    #[serde(alias = "oauthProvider")]
    pub oauth_provider: String,
    #[serde(alias = "oauthUri", deserialize_with = "pdk::serde::deserialize_service")]
    pub oauth_uri: pdk::hl::Service,
    #[serde(alias = "oauthUserBody")]
    pub oauth_user_body: String,
    #[serde(alias = "oauthUserHeader")]
    pub oauth_user_header: String,
}
#[pdk::hl::entrypoint_flex]
fn init(abi: &dyn pdk::flex_abi::api::FlexAbi) -> Result<(), anyhow::Error> {
    let config: Config = serde_json::from_slice(abi.get_configuration())
        .map_err(|err| {
            anyhow::anyhow!(
                "Failed to parse configuration '{}'. Cause: {}",
                String::from_utf8_lossy(abi.get_configuration()), err
            )
        })?;
    abi.service_create(config.oauth_uri)?;
    Ok(())
}
