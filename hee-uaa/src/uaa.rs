use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use hee_core::{config::Config, log};

pub struct Uaa {
    config: Config,
    figment: Figment,
}

impl Uaa {
    pub fn new() -> Self {
        Self::custom(
            Config::figment()
                .merge(Toml::file(Env::var_or("UAA_CONFIG", "conf/uaa.toml")).nested())
                .merge(Env::prefixed("UAA_")),
        )
    }

    pub fn custom<T: figment::Provider>(provider: T) -> Self {
        let (config, figment) = (Config::from(&provider), Figment::from(&provider));
        log::init(config.log_level);

        config.pretty_print(figment.profile());

        Uaa { config, figment }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn figment(&self) -> &Figment {
        &self.figment
    }
}
