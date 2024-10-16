use anyhow::anyhow;
use http::{header::CONTENT_TYPE, HeaderValue};
use reqwest::{
    multipart::{self, Form, Part},
    RequestBuilder,
};
use reqwest_middleware::{ClientWithMiddleware, Middleware};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::errors::RobloxApiError;

#[derive(Clone)]
pub struct MultipartExtension {
    client: &'static ClientWithMiddleware,
    file: Option<(String, PathBuf)>,
    text_parts: BTreeMap<String, String>,
}

impl MultipartExtension {
    pub fn create_multipart(&self) -> reqwest_middleware::Result<Form> {
        let mut form = Form::new();
        if let Some((name, path)) = &self.file {
            // TODO: stream from disk?
            let data = fs::read(&path).map_err(|e| anyhow!(e))?;

            let file_name = path
                .file_name()
                .and_then(OsStr::to_str)
                .ok_or_else(|| RobloxApiError::NoFileName(path.display().to_string()))?
                .to_owned();
            let mime = mime_guess::from_path(&path).first_or_octet_stream();

            let part = Part::bytes(data)
                .file_name(file_name)
                .mime_str(mime.as_ref())
                .unwrap();

            form = form.part(name.clone(), part);
        }
        for (key, value) in self.text_parts.iter() {
            form = form.text(key.clone(), value.clone());
        }
        Ok(form)
    }
}

pub struct MultipartExtensionBuilder {
    inner: MultipartExtension,
}

impl MultipartExtensionBuilder {
    pub fn new(client: &'static ClientWithMiddleware) -> Self {
        MultipartExtensionBuilder {
            inner: MultipartExtension {
                client,
                file: None,
                text_parts: BTreeMap::new(),
            },
        }
    }

    pub fn file(mut self, name: &str, path: &Path) -> Self {
        self.inner.file = Some((name.to_string(), path.to_path_buf()));
        self
    }

    pub fn text(mut self, key: &str, value: &str) -> Self {
        self.inner
            .text_parts
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> MultipartExtension {
        self.inner
    }
}

pub struct MultipartMiddleware;

#[async_trait::async_trait]
impl Middleware for MultipartMiddleware {
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let req = match extensions.get::<MultipartExtension>() {
            Some(ext) => {
                let multipart = ext.create_multipart()?;

                let req = req
                    .try_clone()
                    .ok_or_else(|| anyhow!("Unable to clone request"))?;

                // req.headers_mut().insert(
                //     CONTENT_TYPE,
                //     HeaderValue::from_str(
                //         format!("multipart/form-data; boundary={}", multipart.boundary()).as_str(),
                //     )
                //     .map_err(|e| anyhow!(e))?,
                // );

                RequestBuilder::from_parts(ext.client, req);

                multipart

                // // TODO: can we set content length? multipart.compute_length is private :(

                RequestBuilder::from(req).multipart(multipart).build()
            }
            None => req,
        };
        let res = next.run(req, extensions).await;
        res
    }
}
