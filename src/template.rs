use crate::error::{Error, Result};
use crate::release::Release;
use std::collections::HashMap;
use std::error::Error as ErrorImpl;
use tera::{Context as TeraContext, Result as TeraResult, Tera, Value};

#[derive(Debug)]
pub struct Template {
    tera: Tera,
}

impl Template {
    pub fn new(template: String) -> Result<Self> {
        let mut tera = Tera::default();
        if let Err(e) = tera.add_raw_template("template", &template) {
            return if let Some(error_source) = e.source() {
                Err(Error::TemplateParseError(error_source.to_string()))
            } else {
                Err(Error::TemplateError(e))
            };
        }
        tera.register_filter("upper_first", Self::upper_first_filter);
        Ok(Self { tera })
    }
    /// Filter for making the first character of a string uppercase.
    fn upper_first_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
        let mut s = tera::try_get_value!("upper_first_filter", "value", String, value);
        let mut c = s.chars();
        s = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        Ok(tera::to_value(&s)?)
    }
    pub fn render(&self, release: &Release) -> Result<String> {
        let context = TeraContext::from_serialize(release)?;
        match self.tera.render("template", &context) {
            Ok(v) => Ok(v),
            Err(e) => {
                return if let Some(error_source) = e.source() {
                    Err(Error::TemplateRenderError(error_source.to_string()))
                } else {
                    Err(Error::TemplateError(e))
                };
            }
        }
    }
}
