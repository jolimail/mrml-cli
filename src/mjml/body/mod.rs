use crate::util::{Context, Header, Style};
use roxmltree::Node;
use std::collections::HashMap;

pub mod mj_body;
pub mod mj_button;
pub mod mj_column;
pub mod mj_image;
pub mod mj_section;
pub mod mj_text;
pub mod prelude;
pub mod raw;

use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use prelude::BodyComponent;

#[derive(Clone, Debug)]
pub enum BodyElement {
    MJButton(mj_button::MJButton),
    MJColumn(mj_column::MJColumn),
    MJImage(mj_image::MJImage),
    MJSection(mj_section::MJSection),
    MJText(mj_text::MJText),
    Raw(raw::RawElement),
}

macro_rules! apply_fn {
    ($root:expr, $func:ident($($args:tt)*)) => {
        match $root {
            BodyElement::MJButton(item) => item.$func($($args)*),
            BodyElement::MJColumn(item) => item.$func($($args)*),
            BodyElement::MJImage(item) => item.$func($($args)*),
            BodyElement::MJSection(item) => item.$func($($args)*),
            BodyElement::MJText(item) => item.$func($($args)*),
            BodyElement::Raw(item) => item.$func($($args)*),
        }
    };
}

impl Component for BodyElement {
    fn to_header(&self) -> Header {
        apply_fn!(self, to_header())
    }

    fn context(&self) -> Option<&Context> {
        apply_fn!(self, context())
    }

    fn set_context(&mut self, ctx: Context) {
        apply_fn!(self, set_context(ctx))
    }

    fn render(&self) -> Result<String, Error> {
        apply_fn!(self, render())
    }
}

impl ComponentWithAttributes for BodyElement {
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        apply_fn!(self, source_attributes())
    }

    fn get_attribute(&self, key: &str) -> Option<String> {
        apply_fn!(self, get_attribute(key))
    }
}

impl BodyComponent for BodyElement {
    fn get_style(&self, key: &str) -> Style {
        apply_fn!(self, get_style(key))
    }
}

impl BodyElement {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<BodyElement, Error> {
        let res = match node.tag_name().name() {
            "mj-button" => BodyElement::MJButton(mj_button::MJButton::parse(node)?),
            "mj-column" => BodyElement::MJColumn(mj_column::MJColumn::parse(node)?),
            "mj-image" => BodyElement::MJImage(mj_image::MJImage::parse(node)?),
            "mj-section" => BodyElement::MJSection(mj_section::MJSection::parse(node)?),
            "mj-text" => BodyElement::MJText(mj_text::MJText::parse(node)?),
            _ => BodyElement::Raw(raw::RawElement::parse(node)?),
        };
        Ok(res)
    }
}
