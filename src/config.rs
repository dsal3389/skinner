use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    ops::Range,
    path::PathBuf,
};

#[derive(Deserialize)]
pub struct SessionConfig {
    prompt: String,
    level: u8,
}

#[derive(Deserialize)]
pub struct CommandConfig {
    template_ref: Option<String>,
    level: Option<u8>,
}

#[derive(Deserialize)]
pub struct TemplateConfig {
    pub template: PathBuf,
}

#[derive(Deserialize)]
pub struct RoutesConfig {
    pub count: u32,
    pub template: HashMap<String, TemplateConfig>,
}

#[derive(Deserialize)]
pub struct InterfacesConfig {
    pub count: u32,
    pub name_template: String,
    pub template: HashMap<String, TemplateConfig>,
}

#[derive(Deserialize)]
pub struct Config {
    pub routes: RoutesConfig,
    pub interfaces: InterfacesConfig,
    pub session: HashMap<String, SessionConfig>,
    pub command: HashMap<String, CommandConfig>,
}

pub struct ConfigParser {
    path: PathBuf,
    config: Option<Config>,
}

impl ConfigParser {
    const DEFAULT_CONFIG_PATH: &'static str = "skinner.config.toml";

    pub fn new(path: PathBuf) -> Self {
        let parser = Self { path, config: None };
        parser.create_config_file();
        parser
    }

    pub fn parse(&mut self) -> Result<()> {
        let content = read_to_string(&self.path)
            .with_context(|| format!("reading config file at {:?}", self.path))?;
        let config: Config = toml::from_str(content.as_str()).context("parsing toml file")?;
        self.config = Some(config);
        Ok(())
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    fn create_config_file(&self) {
        if !self.path.exists() {
            let _ = File::create_new(&self.path);
        }
    }
}

impl Default for ConfigParser {
    fn default() -> Self {
        Self::new(Self::DEFAULT_CONFIG_PATH.into())
    }
}
