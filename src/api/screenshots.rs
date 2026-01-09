use napi_derive::napi;

#[napi]
pub mod screenshots {
    /// Triggers the Steam overlay to take a screenshot.
    #[napi]
    pub fn trigger_screenshot() {
        let client = crate::client::get_client();
        let screenshots = client.screenshots();
        screenshots.trigger_screenshot();
    }
}
