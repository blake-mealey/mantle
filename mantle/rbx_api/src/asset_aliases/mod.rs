mod models;

use reqwest::header;
use serde_json::json;

use crate::{
    errors::RobloxApiResult,
    helpers::{handle, handle_as_json},
    models::{AssetId, AssetTypeId},
    RobloxApi,
};

use self::models::{GetAssetAliasResponse, ListAssetAliasesResponse};

impl RobloxApi {
    pub async fn create_asset_alias(
        &self,
        experience_id: AssetId,
        asset_id: AssetId,
        name: String,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post(format!(
                "https://develop.roblox.com/v1/universes/{}/aliases",
                experience_id
            ))
            .json(&json!({
                "name": name,
                "type": "1",
                "targetId": asset_id,
            }));

        handle(req).await?;

        Ok(())
    }

    pub async fn update_asset_alias(
        &self,
        experience_id: AssetId,
        asset_id: AssetId,
        previous_name: String,
        name: String,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post("https://apis.roblox.com/content-aliases-api/v1/universes/update-alias")
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("oldName", &previous_name),
                ("name", &name),
                ("type", &AssetTypeId::Image.to_string()),
                ("targetId", &asset_id.to_string()),
            ]);

        handle(req).await?;

        Ok(())
    }

    pub async fn delete_asset_alias(
        &self,
        experience_id: AssetId,
        name: String,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post("https://apis.roblox.com/content-aliases-api/v1/universes/delete-alias")
            .header(header::CONTENT_LENGTH, 0)
            .query(&[("universeId", &experience_id.to_string()), ("name", &name)]);

        handle(req).await?;

        Ok(())
    }

    pub async fn list_asset_aliases(
        &self,
        experience_id: AssetId,
        page: u32,
    ) -> RobloxApiResult<ListAssetAliasesResponse> {
        let req = self
            .client
            .get("https://api.roblox.com/universes/get-aliases")
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("page", &page.to_string()),
            ]);

        handle_as_json(req).await
    }

    pub async fn get_all_asset_aliases(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<GetAssetAliasResponse>> {
        let mut all_products = Vec::new();

        let mut page: u32 = 1;
        loop {
            let res = self.list_asset_aliases(experience_id, page).await?;
            all_products.extend(res.aliases);

            if res.final_page {
                break;
            }

            page += 1;
        }

        Ok(all_products)
    }
}
