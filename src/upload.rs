use super::access_policy::AccessPolicy;
use super::Gyazo;
use reqwest::multipart;
use std::fs;
use std::path::Path;

/// Gyazo API upload response structure.
#[derive(serde::Deserialize, Debug)]
pub struct UploadResponse {
    /// Timestamp when the image was created.
    pub created_at: String,
    /// Unique identifier of the uploaded image.
    pub image_id: String,
    /// Permalink URL of the image.
    pub permalink_url: String,
    /// URL of the thumbnail.
    pub thumb_url: String,
    /// Type of the response.
    pub r#type: String,
    /// Direct URL to the uploaded image.
    pub url: String,
}

/// Options for uploading images to Gyazo.
///
/// This struct contains optional parameters for configuring image uploads to the Gyazo API.
/// Fields correspond to the API's query parameters, and all fields are optional.
#[derive(Default)]
pub struct GyazoUploadOptions {
    /// Specifies the visibility of the uploaded image.
    /// - `anyone`: The image is visible to anyone with the link. (default)
    /// - `only_me`: The image is visible only to the uploader.
    pub access_policy: Option<AccessPolicy>,

    /// Specifies whether metadata like URL and title is public.
    /// - `"true"`: Metadata is public.
    /// - `"false"`: Metadata is private.
    pub metadata_is_public: Option<bool>,

    /// The URL of the website captured in the image.
    pub referer_url: Option<String>,

    /// The name of the application used to capture the image.
    pub app: Option<String>,

    /// The title of the website captured in the image.
    pub title: Option<String>,

    /// A comment or description for the uploaded image.
    pub desc: Option<String>,

    /// The creation date and time of the image, in Unix time (seconds since the epoch).
    pub created_at: Option<f64>,

    /// The ID of the collection to which the image should be added.
    /// The collection must be owned by or shared with the uploader.
    pub collection_id: Option<String>,
}

impl Gyazo {
    /// Uploads an image to Gyazo.
    ///
    /// # Arguments
    ///
    /// * `image_path` - Path to the image file to upload.
    /// * `options` - Optional upload configuration.
    ///
    /// # Returns
    ///
    /// A `Result` containing `UploadResponse` on success or a `reqwest::Error` on failure.
    // TODO: test
    pub async fn upload<P: AsRef<Path>>(
        &self,
        image_path: P,
        options: Option<&GyazoUploadOptions>,
    ) -> Result<UploadResponse, reqwest::Error> {
        let file_content = fs::read(&image_path).expect("Failed to read the file");

        let mut form = multipart::Form::new()
            .text("access_token", self.access_token.clone())
            .part(
                "imagedata",
                multipart::Part::bytes(file_content)
                    .file_name(image_path.as_ref().to_str().unwrap().to_string()),
            );

        if let Some(opts) = options {
            if let Some(access_policy) = &opts.access_policy {
                form = form.text("access_policy", access_policy.as_str().to_string());
            }
            if let Some(metadata_is_public) = &opts.metadata_is_public {
                form = form.text("metadata_is_public", metadata_is_public.to_string());
            }
            if let Some(referer_url) = &opts.referer_url {
                form = form.text("referer_url", referer_url.clone());
            }
            if let Some(app) = &opts.app {
                form = form.text("app", app.clone());
            }
            if let Some(title) = &opts.title {
                form = form.text("title", title.clone());
            }
            if let Some(desc) = &opts.desc {
                form = form.text("desc", desc.clone());
            }
            if let Some(created_at) = opts.created_at {
                form = form.text("created_at", created_at.to_string());
            }
            if let Some(collection_id) = &opts.collection_id {
                form = form.text("collection_id", collection_id.clone());
            }
        }

        let response = self
            .client
            .post("https://upload.gyazo.com/api/upload")
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json::<UploadResponse>()
            .await?;

        Ok(response)
    }
}
