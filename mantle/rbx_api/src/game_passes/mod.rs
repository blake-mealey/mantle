pub mod models;

use std::path::PathBuf;

use reqwest::multipart::Form;

use crate::{
    errors::RobloxApiResult,
    helpers::{get_file_part, handle, handle_as_json},
    models::AssetId,
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
        let mut req = self
            .client
            .get(format!(
                "https://games.roblox.com/v1/games/{}/game-passes",
                experience_id
            ))
            .query(&[("limit", 100.to_string())]);
        if let Some(page_cursor) = page_cursor {
            req = req.query(&[("cursor", &page_cursor)]);
        }

        handle_as_json(req).await
    }

    pub async fn get_game_pass(
        &self,
        game_pass_id: AssetId,
    ) -> RobloxApiResult<GetGamePassResponse> {
        let req = self.client.get(format!(
            "https://economy.roblox.com/v1/game-pass/{}/game-pass-product-info",
            game_pass_id
        ));

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
        experience_id: AssetId,
        name: String,
        description: String,
        icon_file: PathBuf,
    ) -> RobloxApiResult<CreateGamePassResponse> {
        let req = self
            .client
            .post("https://apis.roblox.com/game-passes/v1/game-passes")
            .multipart(
                Form::new()
                    .text("Name", name.clone())
                    .text("Description", description.clone())
                    .text("UniverseId", experience_id.to_string())
                    .part("File", get_file_part(icon_file).await?),
            );

        handle_as_json(req).await
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
