/// Loads YAML/env settings and exposes typed config to the rest of the app.



mod settings;
pub mod settings;
/// # Errors
///
/// This function will return an error in the following scenarios:
///
/// - **Base Configuration Missing**: The base configuration file (expected at `<CONFIG_PATH>/base`) is not found or cannot be read.
/// - **Invalid Configuration Path**: The `CONFIG_PATH` environment variable is set to an invalid path that cannot be converted to a valid string.
/// - **Configuration Build Failure**: An error occurs while building the configuration from the provided sources, such as syntax errors in configuration files.
/// - **Deserialization Failure**: The assembled configuration cannot be deserialized into the specified `ConfigType`. This may happen due to mismatched types or invalid configuration data.
/// - **Environment Variable Issues**: Required environment variables are missing or have invalid values that affect the configuration assembly.
pub fn get_configuration<ConfigType>(
    maybe_environment: Option<String>,
) -> Result<ConfigType, anyhow::Error>
where
    ConfigType: serde::de::DeserializeOwned,
{
    let mut s = Config::builder();

    let config_path = std::env::var("CONFIG_PATH").unwrap_or("configuration".to_owned());

    let base_path = PathBuf::default().join(config_path);

    #[allow(clippy::shadow_unrelated)]
    let path = base_path.join("base.yaml");

    let path_str = path
        .to_str()
        .ok_or(anyhow::anyhow!("base configuration is not found."))?;
    s = s.add_source(File::with_name(path_str));

    let environment =
        maybe_environment.unwrap_or(env::var("APP_ENVIRONMENT").unwrap_or("test".into()));

    let _full_path = base_path.join(environment);
    s = s.add_source(File::from(path).required(false));

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    s = s.add_source(
        config::Environment::with_prefix("APP")
            .prefix_separator("_")
            .separator("__"),
    );

    s.build()?
        .try_deserialize::<ConfigType>()
        .context("Failed to assemble the required configuration")
}
