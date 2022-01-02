// The MIT License (MIT)
// Copyright © 2021 Aukbit Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Load environment variables into a Config struct
//
// Envy is a library for deserializing environment variables into
// typesafe structs
//
// Dotenv loads environment variables from a .env file, if available,
// and mashes those with the actual environment variables provided by
// the operative system.
//
// Set Config struct into a CONFIG lazy_static to avoid multiple processing.
//
use clap::{App, Arg};
use dotenv;
use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;
use std::env;

// Set Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

/// provides default value for interval if SKIPPER_INTERVAL env var is not set
fn default_interval() -> u64 {
    21600
}

/// provides default value for error interval if SKIPPER_ERROR_INTERVAL env var is not set
fn default_error_interval() -> u64 {
    30
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default = "default_error_interval")]
    pub error_interval: u64,
    pub substrate_ws_url: String,
    pub stashes: Vec<String>,
    #[serde(default)]
    pub is_debug: bool,
    #[serde(default)]
    pub is_short: bool,
    // hooks configuration
    #[serde(default)]
    pub hook_new_session_path: String,
    #[serde(default)]
    pub hook_active_next_era_path: String,
    #[serde(default)]
    pub hook_inactive_next_era_path: String,
    // matrix configuration
    #[serde(default)]
    pub matrix_user: String,
    #[serde(default)]
    pub matrix_bot_user: String,
    #[serde(default)]
    pub matrix_bot_password: String,
    #[serde(default)]
    pub matrix_disabled: bool,
    #[serde(default)]
    pub matrix_bot_display_name_disabled: bool,
}

/// Inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    // Define CLI flags with clap
    let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(
      Arg::with_name("CHAIN")
          .index(1)
          .possible_values(&["westend", "kusama", "polkadot"])
          .help(
            "Sets the substrate-based chain for which 'skipper' will try to connect",
          )
    )
    .arg(
      Arg::with_name("debug")
        .long("debug")
        .help("Prints debug information verbosely."))
    .arg(
      Arg::with_name("matrix-user")
        .long("matrix-user")
        .takes_value(true)
        .help("Your regular matrix user. e.g. '@your-regular-matrix-account:matrix.org' this user account will receive notifications from your other 'Skipper Bot' matrix account."))
    .arg(
          Arg::with_name("matrix-bot-user")
            .long("matrix-bot-user")
            .takes_value(true)
            .help("Your new 'Skipper Bot' matrix user. e.g. '@your-own-skipper-bot-account:matrix.org' this user account will be your 'Skipper Bot' which will be responsible to send messages/notifications to your private or public 'Skipper Bot' rooms."))
    .arg(
      Arg::with_name("matrix-bot-password")
        .long("matrix-bot-password")
        .takes_value(true)
        .help("Password for the 'Skipper Bot' matrix user sign in."))
    .arg(
      Arg::with_name("disable-matrix")
        .long("disable-matrix")
        .help(
          "Disable matrix bot for 'skipper'. (e.g. with this flag active 'skipper' will not send messages/notifications about claimed or unclaimed staking rewards to your private or public 'Skipper Bot' rooms) (https://matrix.org/)",
        ),
    )
    .arg(
      Arg::with_name("disable-public-matrix-room")
        .long("disable-public-matrix-room")
        .help(
          "Disable notifications to matrix public rooms for 'skipper'. (e.g. with this flag active 'skipper' will not send messages/notifications about claimed or unclaimed staking rewards to any public 'Skipper Bot' room)",
        ),
    )
    .arg(
      Arg::with_name("disable-matrix-bot-display-name")
        .long("disable-matrix-bot-display-name")
        .help(
          "Disable matrix bot display name update for 'skipper'. (e.g. with this flag active 'skipper' will not change the matrix bot user display name)",
        ),
      )
    .arg(
      Arg::with_name("short")
        .long("short")
        .help("Display only essential information (e.g. with this flag active 'skipper' will only send essential messages/notifications about claimed rewards)"))
    .arg(
      Arg::with_name("error-interval")
        .long("error-interval")
        .takes_value(true)
        .default_value("30")
        .help("Interval value (in minutes) from which 'skipper' will restart again in case of a critical error."))
    .arg(
      Arg::with_name("stashes")
        .short("s")
        .long("stashes")
        .takes_value(true)
        .help(
          "Validator stash addresses for which 'skipper view', 'skipper' or 'skipper rewards' will be applied. If needed specify more than one (e.g. stash_1,stash_2,stash_3).",
        ),
    )
    .arg(
      Arg::with_name("substrate-ws-url")
        .short("w")
        .long("substrate-ws-url")
        .takes_value(true)
        .help(
          "Substrate websocket endpoint for which 'skipper' will try to connect. (e.g. wss://kusama-rpc.polkadot.io) (NOTE: substrate_ws_url takes precedence than <CHAIN> argument)",
        ),
    )
    .arg(
      Arg::with_name("config-path")
        .short("c")
        .long("config-path")
        .takes_value(true)
        .value_name("FILE")
        .default_value(".env")
        .help(
          "Sets a custom config file path. The config file contains 'skipper' configuration variables.",
        ),
    )
    .arg(
      Arg::with_name("hook-new-session-path")
        .long("hook-new-session-path")
        .takes_value(true)
        .value_name("FILE")
        .help(
          "Sets the path for the script that is called every new session.",
        ),
    )
    .arg(
      Arg::with_name("hook-active-next-era-path")
        .long("hook-active-next-era-path")
        .takes_value(true)
        .value_name("FILE")
        .default_value("./hooks/active_next_era.sh")
        .help(
          "Sets the path for the script that is called on the last session of an era, if the stash is NOT ACTIVE and keys are queued for the next Session/Era.",
        ),
    )
    .arg(
      Arg::with_name("hook-inactive-next-era-path")
        .long("hook-inactive-next-era-path")
        .takes_value(true)
        .value_name("FILE")
        .default_value("./hooks/inactive_next_era.sh")
        .help(
          "Sets the path for the script that is called on the last session of an era, if the stash is active and keys are NOT QUEUED for the next Session/Era.",
        ),
    )
    .get_matches();

    // Try to load configuration from file first
    let config_path = matches.value_of("config-path").unwrap_or(".env");

    match dotenv::from_filename(&config_path).ok() {
        Some(_) => info!("Loading configuration from {} file", &config_path),
        None => {
            let config_path = env::var("SKIPPER_CONFIG_FILENAME").unwrap_or(".env".to_string());
            if let Some(_) = dotenv::from_filename(&config_path).ok() {
                info!("Loading configuration from {} file", &config_path);
            }
        }
    }

    match matches.value_of("CHAIN") {
        Some("westend") => {
            env::set_var("SKIPPER_SUBSTRATE_WS_URL", "wss://westend-rpc.polkadot.io");
        }
        Some("kusama") => {
            env::set_var("SKIPPER_SUBSTRATE_WS_URL", "wss://kusama-rpc.polkadot.io");
        }
        Some("polkadot") => {
            env::set_var("SKIPPER_SUBSTRATE_WS_URL", "wss://rpc.polkadot.io");
        }
        _ => {
            if env::var("SKIPPER_SUBSTRATE_WS_URL").is_err() {
                env::set_var("SKIPPER_SUBSTRATE_WS_URL", "ws://127.0.0.1:9944");
            };
        }
    }

    if let Some(stashes) = matches.value_of("stashes") {
        env::set_var("SKIPPER_STASHES", stashes);
    }

    if let Some(substrate_ws_url) = matches.value_of("substrate-ws-url") {
        env::set_var("SKIPPER_SUBSTRATE_WS_URL", substrate_ws_url);
    }

    if matches.is_present("debug") {
        env::set_var("SKIPPER_IS_DEBUG", "true");
    }

    if matches.is_present("short") {
        env::set_var("SKIPPER_IS_SHORT", "true");
    }

    if let Some(hook_new_session_path) = matches.value_of("hook-new-session-path") {
        env::set_var("SKIPPER_HOOK_NEW_SESSION_PATH", hook_new_session_path);
    }

    if let Some(hook_active_next_era_path) = matches.value_of("hook-active-next-era-path") {
        env::set_var(
            "SKIPPER_HOOK_ACTIVE_NEXT_ERA_PATH",
            hook_active_next_era_path,
        );
    }

    if let Some(hook_inactive_next_era_path) = matches.value_of("hook-inactive-next-era-path") {
        env::set_var(
            "SKIPPER_HOOK_INACTIVE_NEXT_ERA_PATH",
            hook_inactive_next_era_path,
        );
    }

    if matches.is_present("disable-matrix") {
        env::set_var("SKIPPER_MATRIX_DISABLED", "true");
    }

    if matches.is_present("disable-public-matrix-room") {
        env::set_var("SKIPPER_MATRIX_PUBLIC_ROOM_DISABLED", "true");
    }

    if let Some(matrix_user) = matches.value_of("matrix-user") {
        env::set_var("SKIPPER_MATRIX_ACCOUNT", matrix_user);
    }

    if let Some(matrix_bot_user) = matches.value_of("matrix-bot-user") {
        env::set_var("SKIPPER_MATRIX_BOT_USER", matrix_bot_user);
    }

    if let Some(matrix_bot_password) = matches.value_of("matrix-bot-password") {
        env::set_var("SKIPPER_MATRIX_BOT_PASSWORD", matrix_bot_password);
    }

    if let Some(error_interval) = matches.value_of("error-interval") {
        env::set_var("SKIPPER_ERROR_INTERVAL", error_interval);
    }

    match envy::prefixed("SKIPPER_").from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration error: {:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = get_config();
        assert_ne!(config.substrate_ws_url, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.substrate_ws_url, "".to_string());
    }
}
