pub mod models;

use serde_json::json;

use crate::{
    errors::RobloxApiResult,
    helpers::{handle, handle_as_json, handle_as_json_with_status},
    models::AssetId,
    RobloxApi,
};

use self::models::{
    CreatePlaceResponse, GetPlaceResponse, ListPlaceResponse, ListPlacesResponse,
    PlaceConfigurationModel, RemovePlaceResponse,
};

impl RobloxApi {
    pub async fn get_place(&self, place_id: AssetId) -> RobloxApiResult<GetPlaceResponse> {
        let req = self
            .client
            .get(format!("https://develop.roblox.com/v2/places/{}", place_id));

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

    pub async fn create_place(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<CreatePlaceResponse> {
        let req = self
            .client
            .post(format!(
                "https://apis.roblox.com/universes/v1/user/universes/{}/places",
                experience_id
            ))
            .json(&json!({
                "templatePlaceId": 95206881
            }));

        handle_as_json(req).await
    }

    pub async fn configure_place(
        &self,
        place_id: AssetId,
        place_configuration: &PlaceConfigurationModel,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(format!("https://develop.roblox.com/v2/places/{}", place_id))
            .json(place_configuration);

        handle(req).await?;

        Ok(())
    }
}
