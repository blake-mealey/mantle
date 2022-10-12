mod helpers;
pub mod models;

use std::{clone::Clone, ffi::OsStr, fs, path::PathBuf, sync::Arc};

use helpers::{
    get_file_part, get_input_value, handle, handle_as_html, handle_as_json,
    handle_as_json_with_status,
};
use rbx_auth::RobloxAuth;
use reqwest::{header, multipart::Form as MultipartForm, Body};
use serde_json::json;
use thiserror::Error;

use models::*;

// TODO: Improve some of these error messages.
#[derive(Error, Debug)]
pub enum RobloxApiError {
    #[error("HTTP client error.")]
    HttpClient(#[from] reqwest::Error),
    #[error("Authorization has been denied for this request. Check your ROBLOSECURITY cookie.")]
    Authorization,
    #[error("Roblox error: {0}")]
    Roblox(String),
    #[error("Failed to parse JSON response.")]
    ParseJson(#[from] serde_json::Error),
    #[error("Failed to parse HTML response.")]
    ParseHtml,
    #[error("Failed to parse AssetId.")]
    ParseAssetId,
    #[error("Failed to read file.")]
    ReadFile(#[from] std::io::Error),
    #[error("Failed to determine file name for path {0}.")]
    NoFileName(String),
    #[error("Invalid file extension for path {0}.")]
    InvalidFileExtension(String),
    #[error("Failed to read utf8 data.")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("No create quotas found for asset type {0}")]
    MissingCreateQuota(AssetTypeId),
}

// Temporary to make the new errors backwards compatible with the String errors throughout the project.
impl From<RobloxApiError> for String {
    fn from(e: RobloxApiError) -> Self {
        e.to_string()
    }
}

pub type RobloxApiResult<T> = Result<T, RobloxApiError>;

pub struct RobloxApi {
    client: reqwest::Client,
}

impl RobloxApi {
    pub fn new(roblox_auth: RobloxAuth) -> RobloxApiResult<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Roblox/WinInet")
                .cookie_provider(Arc::new(roblox_auth.jar))
                .default_headers(roblox_auth.headers)
                .build()?,
        })
    }

    pub async fn validate_auth(&self) -> RobloxApiResult<()> {
        let req = self
            .client
            .get("https://users.roblox.com/v1/users/authenticated");

        handle(req)
            .await
            .map_err(|_| RobloxApiError::Authorization)?;

        Ok(())
    }

    pub async fn upload_place(
        &self,
        place_file: PathBuf,
        place_id: AssetId,
    ) -> RobloxApiResult<()> {
        let file_format = match place_file.extension().and_then(|e| e.to_str()) {
            Some("rbxl") => PlaceFileFormat::Binary,
            Some("rbxlx") => PlaceFileFormat::Xml,
            _ => {
                return Err(RobloxApiError::InvalidFileExtension(
                    place_file.display().to_string(),
                ))
            }
        };

        let data = fs::read(&place_file)?;

        let body: Body = match file_format {
            PlaceFileFormat::Binary => data.into(),
            PlaceFileFormat::Xml => String::from_utf8(data)?.into(),
        };

        let content_type = match file_format {
            PlaceFileFormat::Binary => "application/octet-stream",
            PlaceFileFormat::Xml => "application/xml",
        };

        let req = self
            .client
            .post("https://data.roblox.com/Data/Upload.ashx")
            .query(&[("assetId", place_id.to_string())])
            .header("Content-Type", content_type)
            .body(body);

        handle(req).await?;

        Ok(())
    }

    pub async fn get_place(&self, place_id: AssetId) -> RobloxApiResult<GetPlaceResponse> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v2/places/{}",
            place_id
        ));

        handle_as_json(req).await
    }

    pub async fn list_places(
        &self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> RobloxApiResult<ListPlacesResponse> {
        let mut req = self.client.get(format!(
            "https://develop.roblox.com/v1/universes/{}/places",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query(&[("cursor", &page_cursor)]);
        }

        handle_as_json(req).await
    }

    // TODO: implement generic form
    pub async fn get_all_places(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<GetPlaceResponse>> {
        let mut all_places = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_places(experience_id, page_cursor).await?;
            for ListPlaceResponse { id } in res.data {
                let place = self.get_place(id).await?;
                all_places.push(place);
            }

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_places)
    }

    pub async fn remove_place_from_experience(
        &self,
        experience_id: AssetId,
        place_id: AssetId,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post("https://www.roblox.com/universes/removeplace")
            .form(&[
                ("universeId", &experience_id.to_string()),
                ("placeId", &place_id.to_string()),
            ]);

        handle_as_json_with_status::<RemovePlaceResponse>(req).await?;

        Ok(())
    }

    pub async fn create_experience(
        &self,
        group_id: Option<AssetId>,
    ) -> RobloxApiResult<CreateExperienceResponse> {
        let req = self
            .client
            .post("https://api.roblox.com/universes/create")
            .json(&json!({
                "templatePlaceIdToUse": 95206881,
                "groupId": group_id
            }));

        handle_as_json(req).await
    }

    pub async fn get_experience(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<GetExperienceResponse> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v1/universes/{}",
            experience_id
        ));

        handle_as_json(req).await
    }

    pub async fn get_experience_configuration(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<ExperienceConfigurationModel> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v1/universes/{}/configuration",
            experience_id
        ));

        handle_as_json(req).await
    }

    pub async fn create_place(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<CreatePlaceResponse> {
        let req = self
            .client
            .post("https://www.roblox.com/ide/places/createV2")
            .header(header::CONTENT_LENGTH, 0)
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("templatePlaceIdToUse", &95206881.to_string()),
            ]);

        handle_as_json_with_status(req).await
    }

    pub async fn configure_experience(
        &self,
        experience_id: AssetId,
        experience_configuration: &ExperienceConfigurationModel,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v2/universes/{}/configuration",
                experience_id
            ))
            .json(experience_configuration);

        handle(req).await?;

        Ok(())
    }

    pub async fn configure_place(
        &self,
        place_id: AssetId,
        place_configuration: &PlaceConfigurationModel,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v2/places/{}",
                place_id
            ))
            .json(place_configuration);

        handle(req).await?;

        Ok(())
    }

    pub async fn set_experience_active(
        &self,
        experience_id: AssetId,
        active: bool,
    ) -> RobloxApiResult<()> {
        let endpoint = if active { "activate" } else { "deactivate" };
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/{}",
                experience_id, endpoint
            ))
            .header(header::CONTENT_LENGTH, 0);

        handle(req).await?;

        Ok(())
    }

    // TODO: Generic form
    pub async fn upload_icon(
        &self,
        experience_id: AssetId,
        icon_file: PathBuf,
    ) -> RobloxApiResult<UploadImageResponse> {
        let req = self
            .client
            .post(&format!(
                "https://publish.roblox.com/v1/games/{}/icon",
                experience_id
            ))
            .multipart(MultipartForm::new().part("request.files", get_file_part(icon_file).await?));

        handle_as_json(req).await
    }

    pub async fn remove_experience_icon(
        &self,
        start_place_id: AssetId,
        icon_asset_id: AssetId,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post("https://www.roblox.com/places/icons/remove-icon")
            .form(&[
                ("placeId", &start_place_id.to_string()),
                ("placeIconId", &icon_asset_id.to_string()),
            ]);

        handle(req).await?;

        Ok(())
    }

    pub async fn upload_thumbnail(
        &self,
        experience_id: AssetId,
        thumbnail_file: PathBuf,
    ) -> RobloxApiResult<UploadImageResponse> {
        let req = self
            .client
            .post(&format!(
                "https://publish.roblox.com/v1/games/{}/thumbnail/image",
                experience_id
            ))
            .multipart(
                MultipartForm::new().part("request.files", get_file_part(thumbnail_file).await?),
            );

        handle_as_json(req).await
    }

    pub async fn get_experience_thumbnails(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<GetExperienceThumbnailResponse>> {
        let req = self.client.get(&format!(
            "https://games.roblox.com/v1/games/{}/media",
            experience_id
        ));

        Ok(handle_as_json::<GetExperienceThumbnailsResponse>(req)
            .await?
            .data)
    }

    pub async fn set_experience_thumbnail_order(
        &self,
        experience_id: AssetId,
        new_thumbnail_order: &[AssetId],
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/thumbnails/order",
                experience_id
            ))
            .json(&json!({ "thumbnailIds": new_thumbnail_order }));

        handle(req).await?;

        Ok(())
    }

    pub async fn delete_experience_thumbnail(
        &self,
        experience_id: AssetId,
        thumbnail_id: AssetId,
    ) -> RobloxApiResult<()> {
        let req = self.client.delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/thumbnails/{}",
            experience_id, thumbnail_id
        ));

        handle(req).await?;

        Ok(())
    }

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
                MultipartForm::new()
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

    pub async fn create_social_link(
        &self,
        experience_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> RobloxApiResult<CreateSocialLinkResponse> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/social-links",
                experience_id
            ))
            .json(&json!({
                "title": title,
                "url": url,
                "type": link_type,
            }));

        handle_as_json(req).await
    }

    pub async fn update_social_link(
        &self,
        experience_id: AssetId,
        social_link_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v1/universes/{}/social-links/{}",
                experience_id, social_link_id
            ))
            .json(&json!({
                "title": title,
                "url": url,
                "type": link_type,
            }));

        handle(req).await?;

        Ok(())
    }

    pub async fn delete_social_link(
        &self,
        experience_id: AssetId,
        social_link_id: AssetId,
    ) -> RobloxApiResult<()> {
        let req = self.client.delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/social-links/{}",
            experience_id, social_link_id
        ));

        handle(req).await?;

        Ok(())
    }

    pub async fn list_social_links(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<GetSocialLinkResponse>> {
        let req = self.client.get(&format!(
            "https://games.roblox.com/v1/games/{}/social-links/list",
            experience_id
        ));

        Ok(handle_as_json::<ListSocialLinksResponse>(req).await?.data)
    }

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
                    MultipartForm::new()
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
                MultipartForm::new()
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
        let mut form = MultipartForm::new()
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

    pub async fn create_badge(
        &self,
        experience_id: AssetId,
        name: String,
        description: String,
        icon_file_path: PathBuf,
        payment_source: CreatorType,
        expected_cost: u32,
    ) -> RobloxApiResult<CreateBadgeResponse> {
        let req = self
            .client
            .post(&format!(
                "https://badges.roblox.com/v1/universes/{}/badges",
                experience_id
            ))
            .multipart(
                MultipartForm::new()
                    .part("request.files", get_file_part(icon_file_path).await?)
                    .text("request.name", name)
                    .text("request.description", description)
                    .text("request.paymentSourceType", payment_source.to_string())
                    .text("request.expectedCost", expected_cost.to_string()),
            );

        handle_as_json(req).await
    }

    pub async fn update_badge(
        &self,
        badge_id: AssetId,
        name: String,
        description: String,
        enabled: bool,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(&format!("https://badges.roblox.com/v1/badges/{}", badge_id))
            .json(&json!({
                "name": name,
                "description": description,
                "enabled": enabled,
            }));

        handle(req).await?;

        Ok(())
    }

    pub async fn get_create_badge_free_quota(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<i32> {
        let req = self.client.get(&format!(
            "https://badges.roblox.com/v1/universes/{}/free-badges-quota",
            experience_id
        ));

        handle_as_json(req).await
    }

    pub async fn list_badges(
        &self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> RobloxApiResult<ListBadgesResponse> {
        let mut req = self.client.get(&format!(
            "https://badges.roblox.com/v1/universes/{}/badges",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query(&[("cursor", &page_cursor)]);
        }

        handle_as_json(req).await
    }

    pub async fn get_all_badges(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<ListBadgeResponse>> {
        let mut all_badges = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_badges(experience_id, page_cursor).await?;
            all_badges.extend(res.data);

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_badges)
    }

    pub async fn update_badge_icon(
        &self,
        badge_id: AssetId,
        icon_file: PathBuf,
    ) -> RobloxApiResult<UploadImageResponse> {
        let req = self
            .client
            .post(&format!(
                "https://publish.roblox.com/v1/badges/{}/icon",
                badge_id
            ))
            .multipart(MultipartForm::new().part("request.files", get_file_part(icon_file).await?));

        handle_as_json(req).await
    }

    pub async fn create_asset_alias(
        &self,
        experience_id: AssetId,
        asset_id: AssetId,
        name: String,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .post(&format!(
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
            .post("https://api.roblox.com/universes/update-alias-v2")
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("oldName", &previous_name),
            ])
            .json(&json!({
                "name": name,
                "type": "1",
                "targetId": asset_id,
            }));

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
            .post("https://api.roblox.com/universes/delete-alias")
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

    pub async fn create_image_asset(
        &self,
        file_path: PathBuf,
        group_id: Option<AssetId>,
    ) -> RobloxApiResult<CreateImageAssetResponse> {
        let data = fs::read(&file_path)?;

        let file_name = format!(
            "Images/{}",
            file_path.file_stem().and_then(OsStr::to_str).unwrap()
        );

        let mut req = self
            .client
            .post("https://data.roblox.com/data/upload/json")
            .header(reqwest::header::CONTENT_TYPE, "*/*")
            .body(data)
            .query(&[
                ("assetTypeId", &AssetTypeId::Decal.to_string()),
                ("name", &file_name),
                ("description", &"madewithmantle".to_owned()),
            ]);
        if let Some(group_id) = group_id {
            req = req.query(&[("groupId", &group_id.to_string())]);
        }

        handle_as_json_with_status(req).await
    }

    pub async fn get_create_asset_quota(
        &self,
        asset_type: AssetTypeId,
    ) -> RobloxApiResult<CreateAssetQuota> {
        let req = self
            .client
            .get("https://publish.roblox.com/v1/asset-quotas")
            .query(&[
                // TODO: Understand what this parameter does
                ("resourceType", "1"),
                ("assetType", &asset_type.to_string()),
            ]);

        // TODO: Understand how to interpret multiple quota objects (rather than just using the first one)
        (handle_as_json::<CreateAssetQuotasResponse>(req).await?)
            .quotas
            .first()
            .cloned()
            .ok_or(RobloxApiError::MissingCreateQuota(asset_type))
    }

    pub async fn create_audio_asset(
        &self,
        file_path: PathBuf,
        group_id: Option<AssetId>,
        payment_source: CreatorType,
    ) -> RobloxApiResult<CreateAudioAssetResponse> {
        let data = fs::read(&file_path)?;

        let file_name = format!(
            "Audio/{}",
            file_path.file_stem().and_then(OsStr::to_str).unwrap()
        );

        let req = self
            .client
            .post("https://publish.roblox.com/v1/audio")
            .json(&json!({
                "name": file_name,
                "file": base64::encode(data),
                "groupId": group_id,
                "paymentSource": payment_source
            }));

        handle_as_json(req).await
    }

    pub async fn archive_asset(&self, asset_id: AssetId) -> RobloxApiResult<()> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/assets/{}/archive",
                asset_id
            ))
            .header(header::CONTENT_LENGTH, 0);

        handle(req).await?;

        Ok(())
    }
}
