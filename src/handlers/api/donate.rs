use crate::{helpers::api_helper::api_success, structs::{api::api::{ApiErrorResult, ApiSuccessResult}, cache::env::ENV_CACHE, donate::donate::DonateAddressResponse}};

pub async fn get_donate_address_api() -> Result<ApiSuccessResult<DonateAddressResponse>, ApiErrorResult> {
    let env = ENV_CACHE.get().unwrap();
    let donate_address = env.donate_address.clone();

    Ok(api_success(DonateAddressResponse {
        address: donate_address,
    })) 
}
