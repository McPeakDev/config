/// ## Config
/// A reusable JSON loader package for applications.
///
/// **NOTE: The struct you plan to parse JSON into must implement Deserialize.**
/// ## Examples
/// ```rust
/// use config;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// pub struct AppConfig {
///     pub test: String,
/// }
///
/// fn main() {
///
///     let config_option: Option<AppConfig> =
///         config::load_config("config.json");
///
///     if config_option.is_some() {
///         let config = config_option.unwrap();
///
///         // Do something with the config here.
///     }
/// }
/// ```
mod config;
pub use crate::config::*;
