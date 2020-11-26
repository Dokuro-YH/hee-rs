mod config;
mod log_level;

pub use self::config::Config;
pub use self::log_level::LogLevel;

#[cfg(test)]
mod tests {
    use figment::Figment;

    use super::*;

    #[test]
    fn test_default_config() {
        let figment = Figment::from(Config::default());

        assert_eq!(figment.profile(), Config::DEFAULT_PROFILE);

        let config: Config = figment.extract().unwrap();

        assert_eq!(config, Config::default());
    }
}
