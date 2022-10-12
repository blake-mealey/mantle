use std::{ffi::OsStr, fs, path::PathBuf};

use reqwest::header;
use serde_json::json;

use crate::{
    errors::{RobloxApiError, RobloxApiResult},
    helpers::{handle, handle_as_json, handle_as_json_with_status},
    models::{
        AssetId, AssetTypeId, CreateAssetQuota, CreateAssetQuotasResponse,
        CreateAudioAssetResponse, CreateImageAssetResponse, CreatorType,
    },
    RobloxApi,
};

impl RobloxApi {
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
