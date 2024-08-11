use anyhow::{Context, Result};
use minijinja::Environment;
use std::env;

mod components;
mod config;
mod terminal;

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

mod generate {
    use anyhow::{Context, Result};
    use minijinja::{context, Environment};
    use rand::random;
    use std::net::{IpAddr, Ipv4Addr};

    use crate::{
        components::{Interface, Route},
        config::{InterfacesConfig, RoutesConfig},
    };

    // used by `next_address` to keep track of which
    // next address is available
    static mut CURRENT_ADDRESS: u32 = 0x01000001;

    // a simple iterator function that returns
    // the next available address based on static var `CURRENT_ADDRESS`
    fn next_address(n: u32) -> Option<IpAddr> {
        unsafe {
            let address = Ipv4Addr::from(CURRENT_ADDRESS);
            CURRENT_ADDRESS += n;

            if address.is_broadcast()
                || address.is_loopback()
                || address.is_multicast()
                || address.is_link_local()
            {
                return None;
            }
            Some(IpAddr::V4(address))
        }
    }

    pub fn interfaces(
        jinja_env: &Environment,
        config: &InterfacesConfig,
    ) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::with_capacity(config.count as usize);
        let name_template = jinja_env
            .get_template(Interface::JINAJ_NAME_TEMP)
            .context(format!(
                "couldn't get jinja template `{}`",
                Interface::JINAJ_NAME_TEMP
            ))?;

        for _ in 0..config.count {
            let address =
                next_address(1).context("couldn't generate more addresses for interfaces")?;
            let name = name_template
                .render(context! (n => 5))
                .context("rendering interface name template")?;
            interfaces.push(Interface::new(name, address));
        }
        Ok(interfaces)
    }

    #[allow(dead_code)]
    pub fn routes<'a>(
        jinja_env: &Environment,
        config: &RoutesConfig,
        interfaces: &'a Vec<Interface>,
    ) -> Result<Vec<Route<'a>>> {
        let mut routes = Vec::new();

        for _ in 0..config.count {
            let address = next_address(1).context("couldn't generate more address for routes")?;
            routes.push(Route::new(address, None));
        }
        Ok(routes)
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

    let interfaces = generate::interfaces(&jinja_env, &config.interfaces)?;
    let routes = generate::routes(&jinja_env, &config.routes, &interfaces)?;

    println!("{routes:?}\n\n");
    println!("{interfaces:?}");
    Ok(())
}
