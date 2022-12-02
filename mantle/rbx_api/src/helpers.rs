use std::{ffi::OsStr, path::PathBuf};

use log::trace;
use reqwest::{multipart::Part, Body};
use scraper::{Html, Selector};
use serde::de;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{errors::RobloxApiErrorResponse, RobloxApiError, RobloxApiResult};

pub async fn get_roblox_api_error_message_from_response(response: reqwest::Response) -> String {
    let status_code = response.status();
    let reason = {
        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if content_type == "application/json" {
                match response.json::<RobloxApiErrorResponse>().await {
                    Ok(error) => Some(error.reason_or_status_code(status_code)),
                    Err(_) => None,
                }
            } else if content_type == "text/html"
                || content_type == "text/html; charset=utf-8"
                || content_type == "text/html; charset=us-ascii"
            {
                match response.text().await {
                    Ok(text) => {
                        let html = Html::parse_fragment(&text);
                        let selector =
                            Selector::parse(".request-error-page-content .error-message").unwrap();

                        html.select(&selector)
                            .next()
                            .map(|e| e.text().map(|t| t.trim()).collect::<Vec<_>>().join(" "))
                    }
                    Err(_) => None,
                }
            } else {
                response.text().await.ok()
            }
        } else {
            None
        }
    };
    reason.unwrap_or_else(|| format!("Unknown error (status {})", status_code))
}

pub async fn handle(
    request_builder: reqwest::RequestBuilder,
) -> RobloxApiResult<reqwest::Response> {
    let result = request_builder.send().await;
    match result {
        Ok(response) => {
            // Check for redirects to the login page
            let url = response.url();
            if matches!(url.domain(), Some("www.roblox.com")) && url.path() == "/NewLogin" {
                return Err(RobloxApiError::Authorization);
            }

            // Check status code
            if response.status().is_success() {
                Ok(response)
            } else {
                Err(RobloxApiError::Roblox(
                    get_roblox_api_error_message_from_response(response).await,
                ))
            }
        }
        Err(error) => Err(error.into()),
    }
}

pub async fn handle_as_json<T>(request_builder: reqwest::RequestBuilder) -> RobloxApiResult<T>
where
    T: de::DeserializeOwned,
{
    let res = handle(request_builder).await?;
    let full = res.text().await?;
    trace!("Handle JSON: {}", full);
    serde_json::from_str::<T>(&full).map_err(|e| e.into())
}

pub async fn handle_as_json_with_status<T>(
    request_builder: reqwest::RequestBuilder,
) -> RobloxApiResult<T>
where
    T: de::DeserializeOwned,
{
    let response = handle(request_builder).await?;
    let status_code = response.status();
    let data = response.bytes().await?;
    if let Ok(error) = serde_json::from_slice::<RobloxApiErrorResponse>(&data) {
        if !error.success.unwrap_or(false) {
            return Err(RobloxApiError::Roblox(
                error.reason_or_status_code(status_code),
            ));
        }
    }
    Ok(serde_json::from_slice::<T>(&data)?)
}

pub async fn handle_as_html(request_builder: reqwest::RequestBuilder) -> RobloxApiResult<Html> {
    let text = handle(request_builder).await?.text().await?;
    Ok(Html::parse_fragment(&text))
}

pub async fn get_file_part(file_path: PathBuf) -> RobloxApiResult<Part> {
    let file = File::open(&file_path).await?;
    let reader = Body::wrap_stream(FramedRead::new(file, BytesCodec::new()));

    let file_name = file_path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| RobloxApiError::NoFileName(file_path.display().to_string()))?
        .to_owned();
    let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

    Ok(Part::stream(reader)
        .file_name(file_name)
        .mime_str(mime.as_ref())
        .unwrap())
}

pub fn get_input_value(html: &Html, selector: &str) -> RobloxApiResult<String> {
    let input_selector = Selector::parse(selector).unwrap();
    let input_element = html
        .select(&input_selector)
        .next()
        .ok_or(RobloxApiError::ParseHtml)?;
    let input_value = input_element
        .value()
        .attr("value")
        .ok_or(RobloxApiError::ParseHtml)?
        .to_owned();

    Ok(input_value)
}
