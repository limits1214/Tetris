use crate::app::config::APP_CONFIG;

pub fn get_aws_config() -> &'static aws_config::SdkConfig {
    &APP_CONFIG.get().unwrap().aws_config
}
