use anyhow::{Context, Result};
use minijinja::Environment;
use std::env;

mod components;
mod config;
mod terminal;

use components::{Interface, Route};
use config::ConfigParser;

mod jinja {
    use crate::{
        components::Interface,
        config::{InterfacesConfig, RoutesConfig, TemplateConfig},
    };
    use anyhow::{Context, Result};
    use minijinja::Environment;
    use std::{collections::HashMap, fs::read_to_string};

    // a simple function that iterates over
    // the config templates and configure them with jinja
    fn setup_component_template(
        jinja_env: &mut Environment,
        template_section_prefix: &str,
        template_config: &HashMap<String, TemplateConfig>,
    ) -> Result<()> {
        for (name, template) in template_config {
            let template_section = format!("{}.{}", template_section_prefix, name);
            let template_content =
                read_to_string(template.template.clone()).with_context(|| {
                    format!(
                        "problem reading interface template for `{}`, path {:?}",
                        template_section, template.template
                    )
                })?;

            // we add the template like the section name in the toml file
            // this will later be easer to to get `template_ref` when to done configuring commands
            jinja_env
                .add_template_owned(template_section.clone(), template_content)
                .with_context(|| {
                    format!("problem parsing jinja template for `{}`", template_section)
                })?;
        }

        Ok(())
    }

    // setup interface jinja templates
    // setup also the interface `name_template`
    // on the jinja env
    pub fn setup_interfaces(
        jinja_env: &mut Environment,
        interfaces_config: &InterfacesConfig,
    ) -> Result<()> {
        jinja_env
            .add_template_owned(
                Interface::JINAJ_NAME_TEMP.to_string(),
                interfaces_config.name_template.clone(),
            )
            .context("setting interface name template")?;
        setup_component_template(
            jinja_env,
            "interfaces.template",
            &interfaces_config.template,
        )?;
        Ok(())
    }

    // setup routes jinjn information
    pub fn setup_routes(jinja_env: &mut Environment, routes_config: &RoutesConfig) -> Result<()> {
        setup_component_template(jinja_env, "routes.template", &routes_config.template)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut jinja_env = Environment::new();
    let mut config_parser = match env::args().nth(1) {
        Some(arg) => ConfigParser::new(arg.into()),
        None => ConfigParser::default(),
    };

    config_parser.parse().context("couldn't parse config")?;

    let config = config_parser.config().context("retriving parsed config")?;
    jinja::setup_interfaces(&mut jinja_env, &config.interfaces)?;
    jinja::setup_routes(&mut jinja_env, &config.routes)?;

    let interfaces = Interface::generaten(&jinja_env, config.interfaces.count)?;
    let routes = Route::generaten(&interfaces, config.routes.count);

    println!("{routes:?}");
    Ok(())
}
