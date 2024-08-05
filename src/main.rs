use anyhow::{Context, Result};
use minijinja::Environment;
use std::{env, fs::read_to_string};

mod components;
mod config;
mod terminal;

use components::Interface;
use config::{ConfigParser, InterfacesConfig, RoutesConfig};

fn jinja_setup_interfaces(
    jinja_env: &mut Environment,
    interfaces_config: &InterfacesConfig,
) -> Result<()> {
    jinja_env
        .add_template_owned(
            Interface::JINAJ_NAME_TEMP.to_string(),
            interfaces_config.name_template.clone(),
        )
        .context("setting interface name template")?;

    for (name, template) in &interfaces_config.template {
        let template_content = read_to_string(template.template.clone()).with_context(|| {
            format!(
                "problem reading interface template for `interface.template.{}`, path {:?}",
                name, template.template
            )
        })?;

        // we add the template like the section name in the toml file
        // this will later be easer to to get `template_ref` when to done configuring commands
        jinja_env
            .add_template_owned(format!("interface.template.{}", name), template_content)
            .with_context(|| {
                format!(
                    "problem parsing jinja template for `interface.template.{}`",
                    name
                )
            })?;
    }
    Ok(())
}

fn jinja_setup_routes(jinja_env: &mut Environment, routes_config: &RoutesConfig) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let mut jinja_env = Environment::new();
    let mut config_parser = match env::args().nth(1) {
        Some(arg) => ConfigParser::new(arg.into()),
        None => ConfigParser::default(),
    };

    config_parser.parse().context("couldn't parse config")?;

    let config = config_parser.config().context("retriving parsed config")?;
    jinja_setup_interfaces(&mut jinja_env, &config.interfaces)?;
    jinja_setup_routes(&mut jinja_env, &config.routes)?;

    let interfaces = Interface::generate_range(&jinja_env, config.interfaces.range.clone())?;

    println!("{interfaces:?}");
    Ok(())
}
