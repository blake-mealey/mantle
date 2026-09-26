pub mod models;

use std::{path::PathBuf, time::Duration};

use reqwest::multipart::Form;

use crate::{
    errors::{RobloxApiError, RobloxApiResult},
    helpers::{get_file_part, handle, handle_as_json},
    models::AssetId,
    RobloxApi,
};

use self::models::{
    CreateDeveloperProductIconResponse, DeveloperProductResponse, ListDeveloperProductsResponse,
};

const ICON_POLL_ATTEMPTS: u32 = 5;
const ICON_POLL_INTERVAL: Duration = Duration::from_secs(1);

impl RobloxApi {
    pub async fn create_developer_product_icon(
        &self,
        experience_id: AssetId,
        developer_product_id: AssetId,
        icon_file: PathBuf,
    ) -> RobloxApiResult<CreateDeveloperProductIconResponse> {
        let previous_icon_id = self
            .get_developer_product(experience_id, developer_product_id)
            .await?
            .icon_image_asset_id;

        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self
                    .client
                    .patch(format!(
                        "https://apis.roblox.com/developer-products/v2/universes/{}/developer-products/{}",
                        experience_id, developer_product_id
                    ))
                    .multipart(Form::new().part("imageFile", get_file_part(&icon_file).await?)))
            })
            .await;
        handle(res).await?;

        let mut icon_id = None;
        for attempt in 0..ICON_POLL_ATTEMPTS {
            if attempt > 0 {
                tokio::time::sleep(ICON_POLL_INTERVAL).await;
            }
            icon_id = self
                .get_developer_product(experience_id, developer_product_id)
                .await?
                .icon_image_asset_id;
            if icon_id.is_some() && icon_id != previous_icon_id {
                break;
            }
        }

        match icon_id {
            Some(image_asset_id) => Ok(CreateDeveloperProductIconResponse { image_asset_id }),
            None => Err(RobloxApiError::MissingCreateDeveloperProductIconResponse),
        }
    }

    pub async fn create_developer_product(
        &self,
        experience_id: AssetId,
        name: String,
        price: u32,
        description: String,
    ) -> RobloxApiResult<DeveloperProductResponse> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self
                    .client
                    .post(format!(
                        "https://apis.roblox.com/developer-products/v2/universes/{}/developer-products",
                        experience_id
                    ))
                    .multipart(
                        Form::new()
                            .text("name", name.clone())
                            .text("description", description.clone())
                            .text("isForSale", "true")
                            .text("price", price.to_string()),
                    ))
            })
            .await;

        handle_as_json(res).await
    }

    pub async fn list_developer_products(
        &self,
        experience_id: AssetId,
        page_token: Option<&str>,
    ) -> RobloxApiResult<ListDeveloperProductsResponse> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                let mut request = self.client.get(format!(
                    "https://apis.roblox.com/developer-products/v2/universes/{}/developer-products/creator",
                    experience_id
                ));

                if let Some(page_token) = page_token {
                    request = request.query(&[("pageToken", page_token)]);
                }

                Ok(request)
            })
            .await;

        handle_as_json(res).await
    }

    pub async fn get_all_developer_products(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<DeveloperProductResponse>> {
        let mut all_products = Vec::new();
        let mut page_token = None;

        loop {
            let res = self
                .list_developer_products(experience_id, page_token.as_deref())
                .await?;
            all_products.extend(res.developer_products);
            page_token = res.next_page_token.filter(|token| !token.is_empty());

            if page_token.is_none() {
                break;
            }
        }

        Ok(all_products)
    }

    pub async fn get_developer_product(
        &self,
        experience_id: AssetId,
        product_id: AssetId,
    ) -> RobloxApiResult<DeveloperProductResponse> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self.client.get(format!(
                    "https://apis.roblox.com/developer-products/v2/universes/{}/developer-products/{}/creator",
                    experience_id, product_id
                )))
            })
            .await;

        handle_as_json(res).await
    }

    pub async fn update_developer_product(
        &self,
        experience_id: AssetId,
        product_id: AssetId,
        name: String,
        price: u32,
        description: String,
    ) -> RobloxApiResult<()> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self
                    .client
                    .patch(format!(
                        "https://apis.roblox.com/developer-products/v2/universes/{}/developer-products/{}",
                        experience_id, product_id
                    ))
                    .multipart(
                        Form::new()
                            .text("name", name.clone())
                            .text("description", description.clone())
                            .text("isForSale", "true")
                            .text("price", price.to_string()),
                    ))
            })
            .await;

        handle(res).await?;
        Ok(())
    }
}
