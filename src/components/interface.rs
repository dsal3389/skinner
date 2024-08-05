use anyhow::{Context, Result};
use minijinja::{context, Environment};
use std::ops::Range;

#[derive(Debug)]
pub struct Interface {
    name: String,
}

impl Interface {
    pub const JINAJ_NAME_TEMP: &'static str = "interface name";

    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn generate_range(jinja_env: &Environment, range: Range<u16>) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        let name_template = jinja_env
            .get_template(Self::JINAJ_NAME_TEMP)
            .context("generating random interfaces")?;

        for _ in range {
            let name = name_template
                .render(context! (n => 5))
                .context("rendering interface name template")?;
            interfaces.push(Interface::new(name));
        }
        Ok(interfaces)
    }
}
