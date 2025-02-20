/// Uniformly operates ZELLIJ* environment variables
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::{set_var, var},
};

pub const ZELLIJ_ENV_KEY: &str = "ZELLIJ";
pub fn get_zellij() -> Result<String> {
    Ok(var(ZELLIJ_ENV_KEY)?)
}
pub fn set_zellij(v: String) {
    set_var(ZELLIJ_ENV_KEY, v);
}

pub const SESSION_NAME_ENV_KEY: &str = "ZELLIJ_SESSION_NAME";

pub fn get_session_name() -> Result<String> {
    Ok(var(SESSION_NAME_ENV_KEY)?)
}

pub fn set_session_name(v: String) {
    set_var(SESSION_NAME_ENV_KEY, v);
}

pub const SOCKET_DIR_ENV_KEY: &str = "ZELLIJ_SOCKET_DIR";
pub fn get_socket_dir() -> Result<String> {
    Ok(var(SOCKET_DIR_ENV_KEY)?)
}

/// Manage ENVIRONMENT VARIABLES from the configuration and the layout files
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariablesFromYaml {
    env: HashMap<String, String>,
}

impl EnvironmentVariablesFromYaml {
    /// Merges two structs, keys from `other` supersede keys from `self`
    pub fn merge(&self, other: Self) -> Self {
        let mut env = self.clone();
        env.env.extend(other.env);
        env
    }

    /// Set all the ENVIRONMENT VARIABLES, that are configured
    /// in the configuration and layout files
    pub fn set_vars(&self) {
        for (k, v) in &self.env {
            set_var(k, v);
        }
    }
}
