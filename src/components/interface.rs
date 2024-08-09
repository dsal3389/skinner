use anyhow::{Context, Result};
use minijinja::{context, Environment};

#[derive(Debug)]
pub struct Interface {
    name: String,
}

impl Interface {
    pub const JINAJ_NAME_TEMP: &'static str = "interface name";

    fn new(name: String) -> Self {
        Self { name }
    }

    // generating `n` random interfaces with jinja information
    pub fn generaten(jinja_env: &Environment, count: u16) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::with_capacity(count as usize);
        let name_template = jinja_env
            .get_template(Self::JINAJ_NAME_TEMP)
            .context(format!(
                "couldn't get jinja template `{}`",
                Self::JINAJ_NAME_TEMP
            ))?;

        for _ in 0..count {
            let name = name_template
                .render(context! (n => 5))
                .context("rendering interface name template")?;
            interfaces.push(Interface::new(name));
        }
        Ok(interfaces)
    }
}
