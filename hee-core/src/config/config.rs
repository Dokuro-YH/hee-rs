use std::net::{IpAddr, Ipv4Addr};

use figment::{
    providers::{Env, Format, Serialized, Toml},
    value::{Dict, Map},
    Error, Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};
use yansi::Paint;

use crate::config::LogLevel;
use crate::log;

#[doc(hidden)]
macro_rules! launch_info { ($($args:tt)*) => { println!($($args)*) } }
#[doc(hidden)]
macro_rules! launch_info_ {
    ($($args:tt)*) => {
        print!("    {} ", Paint::default("=>").bold());
        println!($($args)*);
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub log_level: LogLevel,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: Ipv4Addr::new(127, 0, 0, 1).into(),
            port: 8080,
            log_level: LogLevel::Debug,
        }
    }
}

impl Config {
    pub const DEFAULT_PROFILE: Profile = Profile::const_new("default");

    pub fn from<T: Provider>(provider: T) -> Self {
        Figment::from(provider).extract().unwrap_or_else(|e| {
            pretty_log_error(e);
            panic!("aborting due to configuration error(s)")
        })
    }

    pub fn figment() -> Figment {
        Figment::from(Config::default())
            .merge(Toml::file(Env::var_or("APP_CONFIG", "conf/app.toml")).nested())
            .merge(Env::prefixed("APP_").ignore(&["PROFILE"]).global())
    }

    pub fn pretty_print(&self, profile: &Profile) {
        launch_info!("Configured for {}.", Paint::default(profile).bold());
        launch_info_!("address: {}", Paint::default(&self.address).bold());
        launch_info_!("port: {}", Paint::default(&self.port).bold());
        launch_info_!("log level: {}", Paint::default(self.log_level).bold());
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Application Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        Serialized::defaults(self).data()
    }

    fn profile(&self) -> Option<Profile> {
        Some(Profile::from_env_or("APP_PROFILE", Config::DEFAULT_PROFILE))
    }
}

#[doc(hidden)]
pub fn pretty_log_error(error: figment::Error) {
    crate::log::init(LogLevel::Debug);

    for e in error {
        log::error!("{}", e.kind);

        if let (Some(ref profile), Some(ref md)) = (&e.profile, &e.metadata) {
            if !e.path.is_empty() {
                let key = md.interpolate(profile, &e.path);
                log::error!("for key {}", key);
            }
        }

        if let Some(ref md) = e.metadata {
            if let Some(ref source) = md.source {
                log::error!("in {} {}", source, md.name);
            }
        }
    }
}
