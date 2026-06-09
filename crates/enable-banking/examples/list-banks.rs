use std::{env, fs, time::Duration};

use enable_banking::{
    Client,
    endpoints::{application::GetApplication, aspsps::GetAspsps},
    schemas::{psu_type::PSUType, service::Service},
};
use openssl::pkey::PKey;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    
    let application_id = env::var("APPLICATION_ID")?;
    let private_key_file = env::var("PRIVATE_KEY_PATH")?;

    let raw_private_key = fs::read(private_key_file)?;
    let private_key = PKey::private_key_from_pem(&raw_private_key)?;

    let client = Client::new(application_id, private_key, &Duration::from_mins(1))?;
    
    let response = client.query(&GetApplication).await?;
    dbg!(response);

    let response = client
        .query(&GetAspsps {
            country: Some("ES".to_string()),
            psu_type: Some(PSUType::Personal),
            service: Some(Service::AccountInformationService),
            ..Default::default()
        })
        .await?;
    dbg!(response);
    
    Ok(())
}
