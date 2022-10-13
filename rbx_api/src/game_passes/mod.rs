pub mod models;

use std::path::PathBuf;

use reqwest::multipart::Form;

use crate::{
    errors::{RobloxApiError, RobloxApiResult},
    helpers::{get_file_part, get_input_value, handle, handle_as_html, handle_as_json},
    models::{AssetId, AssetTypeId},
    RobloxApi,
};

use self::models::{
    CreateGamePassResponse, GetGamePassResponse, ListGamePassResponse, ListGamePassesResponse,
};

impl RobloxApi {
    pub async fn list_game_passes(
        &self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> RobloxApiResult<ListGamePassesResponse> {
        let mut req = self.client.get(&format!(
            "https://games.roblox.com/v1/games/{}/game-passes",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query(&[("cursor", &page_cursor)]);
        }

        handle_as_json(req).await
    }

    pub async fn get_game_pass(
        &self,
        game_pass_id: AssetId,
    ) -> RobloxApiResult<GetGamePassResponse> {
        let req = self
            .client
            .get("https://api.roblox.com/marketplace/game-pass-product-info")
            .query(&[("gamePassId", &game_pass_id.to_string())]);

        let mut model = handle_as_json::<GetGamePassResponse>(req).await?;
        if model.target_id == 0 {
            model.target_id = game_pass_id;
        }

        Ok(model)
    }

    pub async fn get_all_game_passes(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<GetGamePassResponse>> {
        let mut all_games = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_game_passes(experience_id, page_cursor).await?;
            for ListGamePassResponse { id } in res.data {
                let game_pass = self.get_game_pass(id).await?;
                all_games.push(game_pass);
            }

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_games)
    }

    pub async fn create_game_pass(
        &self,
        start_place_id: AssetId,
        name: String,
        description: String,
        icon_file: PathBuf,
    ) -> RobloxApiResult<CreateGamePassResponse> {
        let form_verification_token = {
            let req = self
                .client
                .get("https://www.roblox.com/build/upload")
                .query(&[
                    ("assetTypeId", &AssetTypeId::GamePass.to_string()),
                    ("targetPlaceId", &start_place_id.to_string()),
                ]);

            let html = handle_as_html(req).await?;
            get_input_value(
                &html,
                "#upload-form input[name=\"__RequestVerificationToken\"]",
            )?
        };

        let (form_verification_token, icon_asset_id) = {
            let req = self
                .client
                .post("https://www.roblox.com/build/verifyupload")
                .multipart(
                    Form::new()
                        .part("file", get_file_part(icon_file).await?)
                        .text("__RequestVerificationToken", form_verification_token)
                        .text("assetTypeId", AssetTypeId::GamePass.to_string())
                        .text("targetPlaceId", start_place_id.to_string())
                        .text("name", name.clone())
                        .text("description", description.clone()),
                );

            let html = handle_as_html(req).await?;
            let form_verification_token = get_input_value(
                &html,
                "#upload-form input[name=\"__RequestVerificationToken\"]",
            )?;
            let icon_asset_id =
                get_input_value(&html, "#upload-form input[name=\"assetImageId\"]")?
                    .parse::<AssetId>()
                    .map_err(|_| RobloxApiError::ParseAssetId)?;

            (form_verification_token, icon_asset_id)
        };

        let req = self
            .client
            .post("https://www.roblox.com/build/doverifiedupload")
            .multipart(
                Form::new()
                    .text("__RequestVerificationToken", form_verification_token)
                    .text("assetTypeId", AssetTypeId::GamePass.to_string())
                    .text("targetPlaceId", start_place_id.to_string())
                    .text("name", name)
                    .text("description", description)
                    .text("assetImageId", icon_asset_id.to_string()),
            );

        let response = handle(req).await?;

        let location = response.url();
        let asset_id = location
            .query_pairs()
            .find_map(|(k, v)| if k == "uploadedId" { Some(v) } else { None })
            .and_then(|v| v.parse::<AssetId>().ok())
            .ok_or(RobloxApiError::ParseAssetId)?;

        Ok(CreateGamePassResponse {
            asset_id,
            icon_asset_id,
        })
    }

    pub async fn update_game_pass(
        &self,
        game_pass_id: AssetId,
        name: String,
        description: String,
        price: Option<u32>,
        icon_file: Option<PathBuf>,
    ) -> RobloxApiResult<GetGamePassResponse> {
        let mut form = Form::new()
            .text("id", game_pass_id.to_string())
            .text("name", name)
            .text("description", description)
            .text("isForSale", price.is_some().to_string());
        if let Some(price) = price {
            form = form.text("price", price.to_string());
        }
        if let Some(icon_file) = icon_file {
            form = form.part("file", get_file_part(icon_file).await?);
        }

        let req = self
            .client
            .post("https://www.roblox.com/game-pass/update")
            .multipart(form);

        handle(req).await?;

        self.get_game_pass(game_pass_id).await
    }
}
