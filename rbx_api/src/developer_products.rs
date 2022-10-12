use std::path::PathBuf;

use reqwest::{header, multipart::Form};
use serde_json::json;

use crate::{
    errors::{RobloxApiError, RobloxApiResult},
    helpers::{get_file_part, get_input_value, handle, handle_as_html, handle_as_json},
    models::{
        AssetId, CreateDeveloperProductResponse, GetDeveloperProductResponse,
        ListDeveloperProductResponseItem, ListDeveloperProductsResponse,
    },
    RobloxApi,
};

impl RobloxApi {
    pub async fn create_developer_product_icon(
        &self,
        experience_id: AssetId,
        icon_file: PathBuf,
    ) -> RobloxApiResult<AssetId> {
        let image_verification_token = {
            let req = self
                .client
                .get("https://www.roblox.com/places/create-developerproduct")
                .query(&[("universeId", &experience_id.to_string())]);

            let html = handle_as_html(req).await?;
            get_input_value(
                &html,
                "#DeveloperProductImageUpload input[name=\"__RequestVerificationToken\"]",
            )?
        };

        let req = self
            .client
            .post("https://www.roblox.com/places/developerproduct-icon")
            .query(&[("developerProductId", "0")])
            .multipart(
                Form::new()
                    .part("DeveloperProductImageFile", get_file_part(icon_file).await?)
                    .text("__RequestVerificationToken", image_verification_token),
            );

        let html = handle_as_html(req).await?;

        get_input_value(&html, "#developerProductIcon input[id=\"assetId\"]")?
            .parse()
            .map_err(|_| RobloxApiError::ParseAssetId)
    }

    pub async fn create_developer_product(
        &self,
        experience_id: AssetId,
        name: String,
        price: u32,
        description: String,
        icon_asset_id: Option<AssetId>,
    ) -> RobloxApiResult<CreateDeveloperProductResponse> {
        let mut req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/developerproducts",
                experience_id
            ))
            .header(header::CONTENT_LENGTH, 0)
            .query(&[
                ("name", &name),
                ("priceInRobux", &price.to_string()),
                ("description", &description),
            ]);
        if let Some(icon_asset_id) = icon_asset_id {
            req = req.query(&[("iconImageAssetId", &icon_asset_id.to_string())]);
        }

        handle_as_json(req).await
    }

    pub async fn list_developer_products(
        &self,
        experience_id: AssetId,
        page: u32,
    ) -> RobloxApiResult<ListDeveloperProductsResponse> {
        let req = self
            .client
            .get("https://api.roblox.com/developerproducts/list")
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("page", &page.to_string()),
            ]);

        handle_as_json(req).await
    }

    pub async fn get_all_developer_products(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<ListDeveloperProductResponseItem>> {
        let mut all_products = Vec::new();

        let mut page: u32 = 1;
        loop {
            let res = self.list_developer_products(experience_id, page).await?;
            all_products.extend(res.developer_products);

            if res.final_page {
                break;
            }

            page += 1;
        }

        Ok(all_products)
    }

    pub async fn get_developer_product(
        &self,
        developer_product_id: AssetId,
    ) -> RobloxApiResult<GetDeveloperProductResponse> {
        let req = self.client.get(format!(
            "https://develop.roblox.com/v1/developerproducts/{}",
            developer_product_id
        ));

        handle_as_json(req).await
    }

    pub async fn update_developer_product(
        &self,
        experience_id: AssetId,
        product_id: AssetId,
        name: String,
        price: u32,
        description: String,
        icon_asset_id: Option<AssetId>,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/developerproducts/{}/update",
                experience_id, product_id
            ))
            .json(&json!({
                "Name": name,
                "PriceInRobux": price,
                "Description": description,
                "IconImageAssetId": icon_asset_id
            }));

        handle(req).await?;

        Ok(())
    }
}
