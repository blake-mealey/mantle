use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeveloperProductIconResponse {
    pub image_asset_id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperProductResponse {
    pub product_id: AssetId,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon_image_asset_id: Option<AssetId>,
    #[serde(default)]
    pub price_information: DeveloperProductPriceInformation,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperProductPriceInformation {
    #[serde(default)]
    pub default_price_in_robux: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDeveloperProductsResponse {
    pub developer_products: Vec<DeveloperProductResponse>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ListDeveloperProductsResponse;

    #[test]
    fn deserializes_v2_developer_products() {
        let response: ListDeveloperProductsResponse = serde_json::from_str(
            r#"{
                "developerProducts": [{
                    "productId": 123,
                    "name": "Product",
                    "description": "Description",
                    "iconImageAssetId": 456,
                    "priceInformation": { "defaultPriceInRobux": 100 }
                }],
                "nextPageToken": "next"
            }"#,
        )
        .unwrap();

        assert_eq!(response.developer_products[0].product_id, 123);
        assert_eq!(
            response.developer_products[0]
                .price_information
                .default_price_in_robux,
            Some(100)
        );
        assert_eq!(response.next_page_token.as_deref(), Some("next"));
    }
}
